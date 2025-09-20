# Constitution Update Checklist

When amending the constitution (`/memory/constitution.md`), ensure all dependent documents are updated to maintain consistency.

## Templates to Update

### When adding/modifying ANY principle:
- [ ] `/templates/plan-template.md` - Update Constitution Check section
- [ ] `/templates/spec-template.md` - Update if requirements/scope affected
- [ ] `/templates/tasks-template.md` - Update if new task types needed
- [ ] Update documentation references in `.specify/docs/README.md`

### Principle-specific updates:

#### Principle I (Architecture-First Development):
- [ ] Ensure templates reference `.specify/docs/architecture/` documentation
- [ ] Update architecture review requirements in templates
- [ ] Add system design compliance checks

#### Principle II (Documentation-Driven Development):
- [ ] Update documentation requirements in all templates
- [ ] Ensure `.specify/docs/` structure is referenced
- [ ] Add documentation update requirements for all changes

#### Principle III (Security-First Implementation):
- [ ] Update security review requirements in templates
- [ ] Add security compliance checks
- [ ] Ensure JWT and authentication standards are documented

#### Principle IV (Test-Driven Quality Assurance):
- [ ] Update test order and requirements in all templates
- [ ] Emphasize comprehensive testing strategy
- [ ] Add Postman testing integration requirements

#### Principle V (Production-Ready Operations):
- [ ] Update deployment and operations requirements
- [ ] Add Docker and CI/CD integration checks
- [ ] Ensure monitoring and observability standards

## Documentation Structure Updates

### When modifying technical documentation organization:
- [ ] Update `.specify/docs/README.md` navigation
- [ ] Verify all documentation cross-references
- [ ] Update scripts that reference documentation paths
- [ ] Ensure templates point to correct documentation locations

#### Article IV (Integration Testing):
- [ ] List integration test triggers
- [ ] Update test type priorities
- [ ] Add real dependency requirements

#### Article V (Observability):
- [ ] Add logging requirements to templates
- [ ] Include multi-tier log streaming
- [ ] Update performance monitoring sections

#### Article VI (Versioning):
- [ ] Add version increment reminders
- [ ] Include breaking change procedures
- [ ] Update migration requirements

#### Article VII (Simplicity):
- [ ] Update project count limits
- [ ] Add pattern prohibition examples
- [ ] Include YAGNI reminders

## Validation Steps

1. **Before committing constitution changes:**
   - [ ] All templates reference new requirements
   - [ ] Examples updated to match new rules
   - [ ] No contradictions between documents

2. **After updating templates:**
   - [ ] Run through a sample implementation plan
   - [ ] Verify all constitution requirements addressed
   - [ ] Check that templates are self-contained (readable without constitution)

3. **Version tracking:**
   - [ ] Update constitution version number
   - [ ] Note version in template footers
   - [ ] Add amendment to constitution history

## Common Misses

Watch for these often-forgotten updates:
- Command documentation (`/commands/*.md`)
- Checklist items in templates
- Example code/commands
- Domain-specific variations (web vs mobile vs CLI)
- Cross-references between documents

## Template Sync Status

Last sync check: 2025-07-16
- Constitution version: 2.1.1
- Templates aligned: ❌ (missing versioning, observability details)

---

*This checklist ensures the constitution's principles are consistently applied across all project documentation.*