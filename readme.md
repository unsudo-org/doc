```batch
# Fetch latest changes from remote
git fetch origin

# Reset local branch to match remote (destroys local changes)
git reset --hard origin/main

# Remove untracked files/directories (careful!)
git clean -fd

# Clone a remote repository
git clone <url>

# Check repo status
git status

# Add files to staging
git add <file>       # or `git add .` for everything

# Commit staged changes
git commit -m "Message"

# Push to remote
git push origin main

# Pull from remote
git pull origin main

# Create a new branch
git checkout -b new-branch

# Switch to a branch
git checkout main

# List all branches
git branch -a

# Delete a branch
git branch -d local-branch         # Safe delete
git branch -D force-delete-branch  # Force delete

# Push new branch to remote
git push origin new-branch

# Track a remote branch
git checkout --track origin/branch-name

# Unstage a file (undo git add)
git reset <file>

# Reset a file to last commit
git checkout -- <file>

# Soft reset (keep changes)
git reset --soft HEAD~1

# Hard reset (remove local commits and changes)
git reset --hard HEAD~1

# Revert a commit by creating a new inverse commit
git revert <commit-hash>

# Basic log
git log

# One-line summary
git log --oneline

# Graph view
git log --oneline --graph --all --decorate

# Save local changes without committing
git stash

# List stashes
git stash list

# Apply latest stash
git stash apply

# Drop latest stash
git stash drop

# View remotes
git remote -v

# Add remote
git remote add origin <url>

# Change remote URL
git remote set-url origin <new-url>

# Fetch all remotes
git fetch --all

# Create a tag
git tag v1.0

# Push a tag
git push origin v1.0

# List tags
git tag

# Delete local tag
git tag -d v1.0

# Delete remote tag
git push origin --delete tag v1.0

# Add upstream remote
git remote add upstream https://github.com/original/repo.git

# Fetch upstream changes
git fetch upstream

# Merge upstream/main into local main
git checkout main
git merge upstream/main

# Or force override from upstream
git reset --hard upstream/main

# Stage & commit all
git commit -am "Quick commit"

# Delete all local branches except main
git branch | grep -v "main" | xargs git branch -D

# See which branch you're on
git branch

# See remote branches
git branch -r
```