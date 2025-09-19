#!/bin/bash

# Comprehensive Health Check and Monitoring for Docker-based Smart Contract Deployment
# This script provides detailed health monitoring for all deployment components

set -e

# Configuration
MONITOR_LOG="/opt/deployer/logs/health-monitor.log"
STATUS_DIR="/opt/deployer/status"
ARTIFACTS_DIR="/opt/deployer/artifacts"
CONFIG_DIR="/opt/deployer/config"
VALIDATOR_URL="${SOLANA_RPC_URL:-http://solana-validator:8899}"

# Health check levels
HEALTH_OK=0
HEALTH_WARNING=1
HEALTH_CRITICAL=2

# Ensure directories exist
mkdir -p "$(dirname "$MONITOR_LOG")" "$STATUS_DIR"

log() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') [HEALTH] $1" | tee -a "$MONITOR_LOG"
}

# Function to check Solana validator health
check_validator_health() {
    local status=$HEALTH_OK
    local message=""
    
    # Check if validator is responding
    if ! solana cluster-version --url "$VALIDATOR_URL" >/dev/null 2>&1; then
        status=$HEALTH_CRITICAL
        message="Validator not responding to RPC calls"
    else
        # Check block height (should be increasing)
        local block_height
        block_height=$(solana block-height --url "$VALIDATOR_URL" 2>/dev/null || echo "0")
        
        if [ "$block_height" -eq 0 ]; then
            status=$HEALTH_WARNING
            message="Validator responding but not producing blocks"
        else
            # Check if validator is producing blocks (compare with previous height)
            local prev_height_file="$STATUS_DIR/last_block_height.txt"
            if [ -f "$prev_height_file" ]; then
                local prev_height
                prev_height=$(cat "$prev_height_file")
                if [ "$block_height" -le "$prev_height" ]; then
                    status=$HEALTH_WARNING
                    message="Block height not increasing (current: $block_height, previous: $prev_height)"
                else
                    message="Validator healthy (block height: $block_height)"
                fi
            else
                message="Validator healthy (block height: $block_height)"
            fi
            echo "$block_height" > "$prev_height_file"
        fi
    fi
    
    echo "$status|$message"
}

# Function to check deployment status
check_deployment_status() {
    local status=$HEALTH_OK
    local message=""
    
    if [ ! -d "$ARTIFACTS_DIR" ]; then
        status=$HEALTH_CRITICAL
        message="Deployment artifacts directory not found"
    else
        local programs=("registry" "energy-token" "governance" "oracle" "trading")
        local deployed_count=0
        local missing_programs=()
        
        for program in "${programs[@]}"; do
            if [ -f "$ARTIFACTS_DIR/$program/program_id.txt" ]; then
                local program_id
                program_id=$(cat "$ARTIFACTS_DIR/$program/program_id.txt")
                
                # Verify program exists on-chain
                if solana account "$program_id" --url "$VALIDATOR_URL" >/dev/null 2>&1; then
                    deployed_count=$((deployed_count + 1))
                else
                    missing_programs+=("$program")
                fi
            else
                missing_programs+=("$program")
            fi
        done
        
        if [ $deployed_count -eq ${#programs[@]} ]; then
            message="All $deployed_count programs deployed successfully"
        elif [ $deployed_count -gt 0 ]; then
            status=$HEALTH_WARNING
            message="$deployed_count/${#programs[@]} programs deployed (missing: ${missing_programs[*]})"
        else
            status=$HEALTH_CRITICAL
            message="No programs deployed"
        fi
    fi
    
    echo "$status|$message"
}

# Function to check PoA governance status
check_poa_status() {
    local status=$HEALTH_OK
    local message=""
    
    if [ -f "$CONFIG_DIR/poa-status.txt" ]; then
        local poa_status
        poa_status=$(head -1 "$CONFIG_DIR/poa-status.txt")
        
        case "$poa_status" in
            "SUCCESS")
                if [ -f "$ARTIFACTS_DIR/poa-config.json" ]; then
                    local validator_count
                    validator_count=$(jq -r '.validators | length' "$ARTIFACTS_DIR/poa-config.json" 2>/dev/null || echo "0")
                    message="PoA governance initialized with $validator_count REC validators"
                else
                    status=$HEALTH_WARNING
                    message="PoA marked as successful but config not found"
                fi
                ;;
            "FAILED")
                status=$HEALTH_WARNING
                message="PoA governance initialization failed"
                ;;
            *)
                status=$HEALTH_WARNING
                message="PoA status unknown: $poa_status"
                ;;
        esac
    else
        status=$HEALTH_WARNING
        message="PoA governance not initialized"
    fi
    
    echo "$status|$message"
}

# Function to check program functionality
check_program_functionality() {
    local status=$HEALTH_OK
    local message=""
    
    # Check if we can interact with deployed programs
    if [ -f "$ARTIFACTS_DIR/governance/program_id.txt" ]; then
        local governance_program_id
        governance_program_id=$(cat "$ARTIFACTS_DIR/governance/program_id.txt")
        
        # Try to fetch program account info
        if solana account "$governance_program_id" --url "$VALIDATOR_URL" >/dev/null 2>&1; then
            message="Program interactions functional"
        else
            status=$HEALTH_WARNING
            message="Programs deployed but not accessible"
        fi
    else
        status=$HEALTH_CRITICAL
        message="Cannot test program functionality - no deployed programs"
    fi
    
    echo "$status|$message"
}

# Function to check system resources
check_system_resources() {
    local status=$HEALTH_OK
    local message=""
    
    # Check disk space
    local disk_usage
    disk_usage=$(df /opt/deployer 2>/dev/null | awk 'NR==2 {print $5}' | sed 's/%//')
    
    if [ "$disk_usage" -gt 90 ]; then
        status=$HEALTH_CRITICAL
        message="Disk usage critical: ${disk_usage}%"
    elif [ "$disk_usage" -gt 80 ]; then
        status=$HEALTH_WARNING
        message="Disk usage high: ${disk_usage}%"
    else
        message="System resources OK (disk: ${disk_usage}%)"
    fi
    
    echo "$status|$message"
}

# Function to check network connectivity
check_network_connectivity() {
    local status=$HEALTH_OK
    local message=""
    
    # Check if we can reach the validator
    if curl -s -f "$VALIDATOR_URL" >/dev/null 2>&1; then
        message="Network connectivity OK"
    else
        status=$HEALTH_CRITICAL
        message="Cannot reach validator at $VALIDATOR_URL"
    fi
    
    echo "$status|$message"
}

# Function to get overall health status
get_overall_health() {
    local checks=(
        "$(check_validator_health)"
        "$(check_deployment_status)"
        "$(check_poa_status)"
        "$(check_program_functionality)"
        "$(check_system_resources)"
        "$(check_network_connectivity)"
    )
    
    local overall_status=$HEALTH_OK
    local critical_count=0
    local warning_count=0
    local ok_count=0
    
    for check in "${checks[@]}"; do
        local check_status
        check_status=$(echo "$check" | cut -d'|' -f1)
        
        case "$check_status" in
            $HEALTH_CRITICAL)
                critical_count=$((critical_count + 1))
                overall_status=$HEALTH_CRITICAL
                ;;
            $HEALTH_WARNING)
                warning_count=$((warning_count + 1))
                if [ $overall_status -ne $HEALTH_CRITICAL ]; then
                    overall_status=$HEALTH_WARNING
                fi
                ;;
            $HEALTH_OK)
                ok_count=$((ok_count + 1))
                ;;
        esac
    done
    
    echo "$overall_status|$critical_count critical, $warning_count warnings, $ok_count OK"
}

# Function to generate health report
generate_health_report() {
    local verbose="${1:-false}"
    
    echo "=== Smart Contract Deployment Health Report ==="
    echo "Timestamp: $(date)"
    echo "Validator URL: $VALIDATOR_URL"
    echo ""
    
    # Individual checks
    local checks=(
        "Validator Health:$(check_validator_health)"
        "Deployment Status:$(check_deployment_status)"
        "PoA Governance:$(check_poa_status)"
        "Program Functionality:$(check_program_functionality)"
        "System Resources:$(check_system_resources)"
        "Network Connectivity:$(check_network_connectivity)"
    )
    
    for check in "${checks[@]}"; do
        local check_name
        local check_result
        check_name=$(echo "$check" | cut -d':' -f1)
        check_result=$(echo "$check" | cut -d':' -f2-)
        
        local status
        local message
        status=$(echo "$check_result" | cut -d'|' -f1)
        message=$(echo "$check_result" | cut -d'|' -f2-)
        
        case "$status" in
            $HEALTH_OK)
                echo "✅ $check_name: $message"
                ;;
            $HEALTH_WARNING)
                echo "⚠️  $check_name: $message"
                ;;
            $HEALTH_CRITICAL)
                echo "❌ $check_name: $message"
                ;;
        esac
    done
    
    echo ""
    
    # Overall status
    local overall
    overall=$(get_overall_health)
    local overall_status
    local overall_message
    overall_status=$(echo "$overall" | cut -d'|' -f1)
    overall_message=$(echo "$overall" | cut -d'|' -f2-)
    
    case "$overall_status" in
        $HEALTH_OK)
            echo "🟢 Overall Status: HEALTHY ($overall_message)"
            ;;
        $HEALTH_WARNING)
            echo "🟡 Overall Status: WARNING ($overall_message)"
            ;;
        $HEALTH_CRITICAL)
            echo "🔴 Overall Status: CRITICAL ($overall_message)"
            ;;
    esac
    
    # Additional details if verbose
    if [ "$verbose" = "true" ]; then
        echo ""
        echo "=== Detailed Information ==="
        
        # Show deployed programs
        if [ -d "$ARTIFACTS_DIR" ]; then
            echo ""
            echo "Deployed Programs:"
            for program_dir in "$ARTIFACTS_DIR"/*/; do
                if [ -d "$program_dir" ]; then
                    local program_name
                    program_name=$(basename "$program_dir")
                    if [ -f "$program_dir/program_id.txt" ]; then
                        local program_id
                        program_id=$(cat "$program_dir/program_id.txt")
                        echo "  - $program_name: $program_id"
                    fi
                fi
            done
        fi
        
        # Show PoA configuration
        if [ -f "$ARTIFACTS_DIR/poa-config.json" ]; then
            echo ""
            echo "PoA Configuration:"
            jq -r '.validators[] | "  - \(.authorityName): \(.pubkey)"' "$ARTIFACTS_DIR/poa-config.json" 2>/dev/null || \
                echo "  Error reading PoA configuration"
        fi
    fi
    
    echo ""
    echo "=== End of Health Report ==="
    
    return $overall_status
}

# Function to monitor and alert
monitor_and_alert() {
    local alert_threshold="${1:-$HEALTH_WARNING}"
    
    local overall
    overall=$(get_overall_health)
    local overall_status
    overall_status=$(echo "$overall" | cut -d'|' -f1)
    
    if [ $overall_status -ge $alert_threshold ]; then
        log "ALERT: Health check failed with status $overall_status"
        generate_health_report true | tee -a "$MONITOR_LOG"
        
        # Save alert status
        echo "ALERT|$(date)|$overall_status|$(echo "$overall" | cut -d'|' -f2-)" > "$STATUS_DIR/last_alert.txt"
        
        return $overall_status
    else
        log "Health check passed"
        echo "OK|$(date)|$overall_status|$(echo "$overall" | cut -d'|' -f2-)" > "$STATUS_DIR/last_check.txt"
        return 0
    fi
}

# Main function
main() {
    case "${1:-check}" in
        "check")
            generate_health_report false
            ;;
        "detailed")
            generate_health_report true
            ;;
        "monitor")
            monitor_and_alert $HEALTH_WARNING
            ;;
        "alert")
            monitor_and_alert $HEALTH_CRITICAL
            ;;
        "status")
            local overall
            overall=$(get_overall_health)
            echo "$overall"
            ;;
        *)
            echo "Usage: $0 {check|detailed|monitor|alert|status}"
            echo ""
            echo "Commands:"
            echo "  check     - Basic health check report"
            echo "  detailed  - Detailed health check report"
            echo "  monitor   - Monitor and alert on warnings"
            echo "  alert     - Monitor and alert only on critical issues"
            echo "  status    - Return status code only"
            exit 1
            ;;
    esac
}

# Execute main function
main "$@"