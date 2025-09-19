#!/bin/bash

# Build all Anchor programs for the P2P Energy Trading System
# This script handles the build process for all 5 programs

set -e

WORKSPACE_DIR="/opt/deployer/workspace"
ARTIFACTS_DIR="/opt/deployer/artifacts"
LOG_FILE="/opt/deployer/logs/build.log"

# Ensure log directory exists
mkdir -p /opt/deployer/logs

log() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') [BUILD] $1" | tee -a "$LOG_FILE"
}

# Function to build a single program
build_program() {
    local program_name="$1"
    local program_path="$WORKSPACE_DIR/programs/$program_name"
    
    log "Building program: $program_name"
    
    if [ ! -d "$program_path" ]; then
        log "ERROR: Program directory not found: $program_path"
        return 1
    fi
    
    cd "$program_path"
    
    # Check for Anchor.toml
    if [ ! -f "Anchor.toml" ]; then
        log "WARNING: No Anchor.toml found in $program_path, checking parent directory"
        cd "$WORKSPACE_DIR"
        if [ ! -f "Anchor.toml" ]; then
            log "ERROR: No Anchor.toml found in workspace"
            return 1
        fi
    fi
    
    # Clean previous build artifacts
    log "Cleaning previous build artifacts for $program_name"
    if [ -d "target" ]; then
        rm -rf target
    fi
    
    # Build the program
    log "Running anchor build for $program_name"
    if ! anchor build --program-name "$program_name" 2>&1 | tee -a "$LOG_FILE"; then
        log "ERROR: Failed to build $program_name"
        return 1
    fi
    
    # Copy artifacts
    local target_dir="$WORKSPACE_DIR/target"
    if [ -d "$target_dir" ]; then
        log "Copying build artifacts for $program_name"
        mkdir -p "$ARTIFACTS_DIR/$program_name"
        
        # Copy IDL files
        if [ -d "$target_dir/idl" ]; then
            cp -r "$target_dir/idl"/* "$ARTIFACTS_DIR/$program_name/" 2>/dev/null || true
        fi
        
        # Copy program binaries
        if [ -d "$target_dir/deploy" ]; then
            cp -r "$target_dir/deploy"/* "$ARTIFACTS_DIR/$program_name/" 2>/dev/null || true
        fi
        
        # Copy types
        if [ -d "$target_dir/types" ]; then
            cp -r "$target_dir/types"/* "$ARTIFACTS_DIR/$program_name/" 2>/dev/null || true
        fi
    fi
    
    log "Successfully built $program_name"
    return 0
}

# Main build function
main() {
    log "=== Starting build process for all contracts ==="
    
    # Check if workspace exists
    if [ ! -d "$WORKSPACE_DIR" ]; then
        log "ERROR: Workspace directory not found: $WORKSPACE_DIR"
        exit 1
    fi
    
    # Define build order (dependencies first)
    local programs=("registry" "energy-token" "governance" "oracle" "trading")
    local built_programs=()
    local failed_programs=()
    
    # Clean artifacts directory
    rm -rf "$ARTIFACTS_DIR"
    mkdir -p "$ARTIFACTS_DIR"
    
    # Build all programs
    for program in "${programs[@]}"; do
        if build_program "$program"; then
            built_programs+=("$program")
        else
            failed_programs+=("$program")
        fi
    done
    
    # Report build results
    log "Build Summary:"
    log "Successfully built (${#built_programs[@]}): ${built_programs[*]}"
    
    if [ ${#failed_programs[@]} -gt 0 ]; then
        log "Failed to build (${#failed_programs[@]}): ${failed_programs[*]}"
        log "=== Build process failed ==="
        exit 1
    fi
    
    log "All programs built successfully"
    log "Build artifacts saved to: $ARTIFACTS_DIR"
    log "=== Build process completed successfully ==="
    
    return 0
}

# Execute main function
main "$@"