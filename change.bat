@echo off
:: ==========================================
:: CONFIGURATION: Your PERSONAL details
:: ==========================================
set CORRECT_NAME=Thomas-Hutch
set CORRECT_EMAIL=htchnstm@gmail.com

:: ==========================================
:: IGNORE WARNINGS
:: ==========================================
set FILTER_BRANCH_SQUELCH_WARNING=1

:: ==========================================
:: EXECUTE UNCONDITIONAL REWRITE
:: ==========================================
echo Overwriting ALL history with new author...

git filter-branch --force --env-filter "export GIT_COMMITTER_NAME='%CORRECT_NAME%'; export GIT_COMMITTER_EMAIL='%CORRECT_EMAIL%'; export GIT_AUTHOR_NAME='%CORRECT_NAME%'; export GIT_AUTHOR_EMAIL='%CORRECT_EMAIL%'" --tag-name-filter cat -- --branches --tags

echo.
echo ---------------------------------------------------------
echo Rewrite complete.
echo Run: git push --force origin main
echo ---------------------------------------------------------
pause