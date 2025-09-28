#!/bin/bash

# Automated GitHub Issue Creation Script
# This script will help you quickly create all 5 starter issues

echo "🚀 Pico Community Toolkit - Automated Issue Creation"
echo "=================================================="
echo ""

# Check if GitHub CLI is installed
if command -v gh &> /dev/null; then
    echo "✅ GitHub CLI detected! Using automated creation..."
    echo ""
    
    # Create Issue 1: Status Badges
    echo "Creating Issue 1: Status Badges..."
    gh issue create --title "[GOOD FIRST ISSUE] Add status badges to main README" \
      --body-file .github/ISSUES/issue-1-status-badges.md \
      --label "good first issue,documentation"
    echo "✅ Issue 1 created!"
    echo ""
    
    # Create Issue 2: Error Messages
    echo "Creating Issue 2: Error Messages..."
    gh issue create --title "[GOOD FIRST ISSUE] Improve error messages in CLI tools" \
      --body-file .github/ISSUES/issue-2-error-messages.md \
      --label "good first issue,enhancement,cli-extensions"
    echo "✅ Issue 2 created!"
    echo ""
    
    # Create Issue 3: Test Cases
    echo "Creating Issue 3: Test Cases..."
    gh issue create --title "[GOOD FIRST ISSUE] Add comprehensive test cases" \
      --body-file .github/ISSUES/issue-3-test-cases.md \
      --label "good first issue,testing,testing-framework"
    echo "✅ Issue 3 created!"
    echo ""
    
    # Create Issue 4: Video Tutorials
    echo "Creating Issue 4: Video Tutorials..."
    gh issue create --title "[DOCUMENTATION] Create video tutorials for toolkit components" \
      --body-file .github/ISSUES/issue-4-video-tutorials.md \
      --label "documentation,learning-academy,help wanted"
    echo "✅ Issue 4 created!"
    echo ""
    
    # Create Issue 5: Configuration Files
    echo "Creating Issue 5: Configuration Files..."
    gh issue create --title "[ENHANCEMENT] Add configuration file support for CLI tools" \
      --body-file .github/ISSUES/issue-5-config-files.md \
      --label "enhancement,cli-extensions,configuration"
    echo "✅ Issue 5 created!"
    echo ""
    
    echo "🎉 All 5 issues created successfully!"
    echo "Visit: https://github.com/SemiuAdesina/pico-community-toolkit/issues"
    
else
    echo "❌ GitHub CLI not found. Using manual creation method..."
    echo ""
    echo "📋 Manual Issue Creation Instructions:"
    echo "====================================="
    echo ""
    echo "1. Go to: https://github.com/SemiuAdesina/pico-community-toolkit/issues"
    echo "2. Click 'New issue'"
    echo "3. Copy and paste the content from the files below:"
    echo ""
    
    # Display issue contents for manual copying
    echo "📝 ISSUE 1: Status Badges"
    echo "Title: [GOOD FIRST ISSUE] Add status badges to main README"
    echo "Labels: good first issue, documentation"
    echo "Content:"
    echo "----------------------------------------"
    cat .github/ISSUES/issue-1-status-badges.md
    echo ""
    echo "----------------------------------------"
    echo ""
    echo "Press Enter to continue to Issue 2..."
    read -r
    
    echo "📝 ISSUE 2: Error Messages"
    echo "Title: [GOOD FIRST ISSUE] Improve error messages in CLI tools"
    echo "Labels: good first issue, enhancement, cli-extensions"
    echo "Content:"
    echo "----------------------------------------"
    cat .github/ISSUES/issue-2-error-messages.md
    echo ""
    echo "----------------------------------------"
    echo ""
    echo "Press Enter to continue to Issue 3..."
    read -r
    
    echo "📝 ISSUE 3: Test Cases"
    echo "Title: [GOOD FIRST ISSUE] Add comprehensive test cases"
    echo "Labels: good first issue, testing, testing-framework"
    echo "Content:"
    echo "----------------------------------------"
    cat .github/ISSUES/issue-3-test-cases.md
    echo ""
    echo "----------------------------------------"
    echo ""
    echo "Press Enter to continue to Issue 4..."
    read -r
    
    echo "📝 ISSUE 4: Video Tutorials"
    echo "Title: [DOCUMENTATION] Create video tutorials for toolkit components"
    echo "Labels: documentation, learning-academy, help wanted"
    echo "Content:"
    echo "----------------------------------------"
    cat .github/ISSUES/issue-4-video-tutorials.md
    echo ""
    echo "----------------------------------------"
    echo ""
    echo "Press Enter to continue to Issue 5..."
    read -r
    
    echo "📝 ISSUE 5: Configuration Files"
    echo "Title: [ENHANCEMENT] Add configuration file support for CLI tools"
    echo "Labels: enhancement, cli-extensions, configuration"
    echo "Content:"
    echo "----------------------------------------"
    cat .github/ISSUES/issue-5-config-files.md
    echo ""
    echo "----------------------------------------"
    echo ""
    echo "🎉 All issue content displayed!"
    echo "Visit: https://github.com/SemiuAdesina/pico-community-toolkit/issues"
fi

echo ""
echo "📊 Repository Status After Issues:"
echo "- 3 Good First Issues (beginner-friendly)"
echo "- 1 Documentation Issue (video tutorials)"
echo "- 1 Enhancement Issue (configuration files)"
echo "- Professional issue templates"
echo "- Clear contribution guidelines"
echo ""
echo "🚀 Your repository is now ready to attract contributors and trend!"
