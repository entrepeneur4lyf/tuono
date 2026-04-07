```markdown
# tuono Development Patterns

> Auto-generated skill from repository analysis

## Overview
This skill teaches you the core development patterns and conventions used in the `tuono` TypeScript codebase. You'll learn about file naming, import/export styles, commit message conventions, and how to write and organize tests. This guide is designed to help you contribute effectively and consistently to the project.

## Coding Conventions

### File Naming
- Use **camelCase** for file names.
  - Example: `myComponent.ts`, `userService.ts`

### Imports
- Use **relative imports** for referencing modules within the project.
  - Example:
    ```typescript
    import { fetchData } from './apiClient';
    ```

### Exports
- Use **named exports** for all modules.
  - Example:
    ```typescript
    // In userService.ts
    export function getUser(id: string) { ... }
    export const USER_ROLE = 'admin';
    ```

### Commit Messages
- Use **conventional commit** format.
- Prefix with `chore` for maintenance tasks.
- Keep commit messages concise (average ~42 characters).
  - Example:
    ```
    chore: update dependencies to latest versions
    ```

## Workflows

### Code Contribution
**Trigger:** When adding or modifying code in the repository  
**Command:** `/contribute`

1. Create a new branch for your feature or fix.
2. Follow the camelCase naming convention for new files.
3. Use relative imports and named exports in your code.
4. Write or update tests in files matching `*.test.*`.
5. Commit changes using the conventional commit format (e.g., `chore: ...`).
6. Open a pull request for review.

### Testing
**Trigger:** When verifying code changes  
**Command:** `/test`

1. Identify or create test files named with the pattern `*.test.*`.
2. Add or update tests relevant to your changes.
3. Run the test suite using the project's test runner (framework unknown; refer to project scripts or documentation).
4. Ensure all tests pass before submitting your changes.

## Testing Patterns

- Test files are named using the pattern `*.test.*` (e.g., `userService.test.ts`).
- The specific testing framework is not detected; check project scripts or documentation for details.
- Place tests alongside the code they test or in a dedicated `tests` directory as per project structure.

**Example:**
```typescript
// userService.test.ts
import { getUser } from './userService';

describe('getUser', () => {
  it('should return a user object for a valid ID', () => {
    const user = getUser('123');
    expect(user).toBeDefined();
  });
});
```

## Commands
| Command      | Purpose                                             |
|--------------|-----------------------------------------------------|
| /contribute  | Guide for contributing code to the repository       |
| /test        | Steps to write and run tests                        |
```
