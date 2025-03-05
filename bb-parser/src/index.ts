import { chromium, ChromiumBrowser, Page } from 'playwright';

const BITBUCKET_USER = 'ВАШ_ЛОГИН';
const BITBUCKET_PASS = 'ВАШ_ПАРОЛЬ';

interface CommentsResult {
  [key: string]: string[];
}

async function findMyComments(prUrls: string[]): Promise<CommentsResult> {
  const browser: ChromiumBrowser = await chromium.launch({ headless: false });
  const page: Page = await browser.newPage();
  
  // Авторизация в Bitbucket
  await page.goto('https://bitbucket.org/account/signin/');
  await page.fill('input[name="username"]', BITBUCKET_USER);
  await page.click('button:has-text("Continue")');
  await page.fill('input[name="password"]', BITBUCKET_PASS);
  await page.click('button:has-text("Log in")');
  await page.waitForURL('https://bitbucket.org/dashboard/overview**');

  const results: CommentsResult = {};

  for (const prUrl of prUrls) {
    await page.goto(prUrl);
    await page.waitForSelector('.comment');
    
    const comments = await page.$$('.comment');
    const myComments: string[] = [];
    
    for (const comment of comments) {
      const authorElement = await comment.$('.author-name');
      const authorName = await authorElement?.innerText();
      
      if (authorName?.trim() === BITBUCKET_USER) {
        const contentElement = await comment.$('.comment-content');
        const content = await contentElement?.innerText();
        if (content) myComments.push(content.trim());
      }
    }
    
    results[prUrl] = myComments;
  }

  await browser.close();
  return results;
}

// Запуск скрипта
const prUrls = process.argv.slice(2);
if (prUrls.length === 0) {
  console.error('Usage: ts-node script.ts <PR_URL1> <PR_URL2> ...');
  process.exit(1);
}

findMyComments(prUrls)
  .then(results => {
    for (const [url, comments] of Object.entries(results)) {
      console.log(`\nPR: ${url}`);
      console.log(`Найдено комментариев: ${comments.length}`);
      comments.forEach((comment, index) => {
        console.log(`${index + 1}. ${comment}`);
      });
    }
  })
  .catch(console.error);