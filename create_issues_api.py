#!/usr/bin/env python3
"""
Automated GitHub Issue Creation Script
Creates all 5 starter issues for the Pico Community Toolkit
"""

import os
import sys
import requests
import json
from pathlib import Path

# Configuration
REPO_OWNER = "SemiuAdesina"
REPO_NAME = "pico-community-toolkit"
GITHUB_API_URL = f"https://api.github.com/repos/{REPO_OWNER}/{REPO_NAME}/issues"

# Issue configurations
ISSUES = [
    {
        "title": "[GOOD FIRST ISSUE] Add status badges to main README",
        "labels": ["good first issue", "documentation"],
        "body_file": ".github/ISSUES/issue-1-status-badges.md"
    },
    {
        "title": "[GOOD FIRST ISSUE] Improve error messages in CLI tools",
        "labels": ["good first issue", "enhancement", "cli-extensions"],
        "body_file": ".github/ISSUES/issue-2-error-messages.md"
    },
    {
        "title": "[GOOD FIRST ISSUE] Add comprehensive test cases",
        "labels": ["good first issue", "testing", "testing-framework"],
        "body_file": ".github/ISSUES/issue-3-test-cases.md"
    },
    {
        "title": "[DOCUMENTATION] Create video tutorials for toolkit components",
        "labels": ["documentation", "learning-academy", "help wanted"],
        "body_file": ".github/ISSUES/issue-4-video-tutorials.md"
    },
    {
        "title": "[ENHANCEMENT] Add configuration file support for CLI tools",
        "labels": ["enhancement", "cli-extensions", "configuration"],
        "body_file": ".github/ISSUES/issue-5-config-files.md"
    }
]

def get_github_token():
    """Get GitHub token from environment variable or prompt user"""
    token = os.getenv('GITHUB_TOKEN')
    if not token:
        print("🔑 GitHub Token Required")
        print("You need a GitHub Personal Access Token to create issues via API.")
        print("")
        print("1. Go to: https://github.com/settings/tokens")
        print("2. Click 'Generate new token (classic)'")
        print("3. Select 'repo' scope")
        print("4. Copy the token")
        print("")
        token = input("Enter your GitHub token: ").strip()
        if not token:
            print("❌ No token provided. Exiting.")
            sys.exit(1)
    return token

def read_issue_body(file_path):
    """Read issue body from markdown file"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
            # Remove the YAML front matter (--- lines and content between them)
            lines = content.split('\n')
            body_lines = []
            in_frontmatter = False
            
            for line in lines:
                if line.strip() == '---':
                    in_frontmatter = not in_frontmatter
                    continue
                if not in_frontmatter:
                    body_lines.append(line)
            
            return '\n'.join(body_lines).strip()
    except FileNotFoundError:
        print(f"❌ Issue body file not found: {file_path}")
        return "Issue body content not found."

def create_issue(token, issue_config):
    """Create a single issue via GitHub API"""
    headers = {
        "Authorization": f"token {token}",
        "Accept": "application/vnd.github.v3+json",
        "Content-Type": "application/json"
    }
    
    # Read issue body
    body = read_issue_body(issue_config["body_file"])
    
    # Prepare issue data
    issue_data = {
        "title": issue_config["title"],
        "body": body,
        "labels": issue_config["labels"]
    }
    
    # Create issue
    response = requests.post(GITHUB_API_URL, headers=headers, json=issue_data)
    
    if response.status_code == 201:
        issue = response.json()
        print(f"✅ Created: {issue['title']}")
        print(f"   URL: {issue['html_url']}")
        return True
    else:
        print(f"❌ Failed to create: {issue_config['title']}")
        print(f"   Status: {response.status_code}")
        print(f"   Error: {response.text}")
        return False

def main():
    """Main function"""
    print("🚀 Pico Community Toolkit - Automated Issue Creation")
    print("=" * 55)
    print("")
    
    # Check if issue files exist
    missing_files = []
    for issue in ISSUES:
        if not Path(issue["body_file"]).exists():
            missing_files.append(issue["body_file"])
    
    if missing_files:
        print("❌ Missing issue body files:")
        for file in missing_files:
            print(f"   - {file}")
        print("")
        print("Please run this script from the repository root directory.")
        sys.exit(1)
    
    # Get GitHub token
    token = get_github_token()
    print("")
    
    # Create issues
    print("📝 Creating issues...")
    print("")
    
    success_count = 0
    for i, issue_config in enumerate(ISSUES, 1):
        print(f"[{i}/{len(ISSUES)}] Creating: {issue_config['title']}")
        if create_issue(token, issue_config):
            success_count += 1
        print("")
    
    # Summary
    print("📊 Summary:")
    print(f"✅ Successfully created: {success_count}/{len(ISSUES)} issues")
    print(f"❌ Failed to create: {len(ISSUES) - success_count}/{len(ISSUES)} issues")
    print("")
    
    if success_count > 0:
        print("🎉 Issues created successfully!")
        print(f"Visit: https://github.com/{REPO_OWNER}/{REPO_NAME}/issues")
        print("")
        print("📈 Your repository now has:")
        print("- 3 Good First Issues (beginner-friendly)")
        print("- 1 Documentation Issue (video tutorials)")
        print("- 1 Enhancement Issue (configuration files)")
        print("- Professional issue templates")
        print("- Clear contribution guidelines")
        print("")
        print("🚀 Your repository is now ready to attract contributors and trend!")

if __name__ == "__main__":
    main()
