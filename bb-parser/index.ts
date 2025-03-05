import { chromium, ChromiumBrowser, Page } from "playwright";
import { readFileSync } from "fs";
import { resolve } from "path";

interface CommentsResult {
  [key: string]: string[];
}

function readPrUrlsFromFile(filePath = "list"): string[] {
  try {
    const fullPath = resolve(process.cwd(), filePath);
    const content = readFileSync(fullPath, "utf-8");

    return content
      .split("\n")
      .map((line) => line.trim())
      .filter((line) => line.length > 0 && line.startsWith("http"));
  } catch (error) {
    console.error(`Error reading file ${filePath}:`);
    console.error((error as Error).message);
    process.exit(1);
  }
}

async function loadAllComments(
  page: Page,
  username: string
): Promise<string[]> {
  let previousHeight = 0;
  let attempts = 0;
  const maxAttempts = 5;
  const comments = new Set<string>();

  while (attempts < maxAttempts) {
    // Скролл вниз
    await page.evaluate(() => {
      window.scrollTo(0, document.body.scrollHeight);
    });

    // Ожидание через waitForTimeout вместо setTimeout
    await page.waitForTimeout(2000);

    // Проверка новых комментариев
    const currentComments = await page.$$eval(
      ".comment",
      (nodes, user) => {
        return nodes
          .filter(
            (node) =>
              node.querySelector(".user-name")?.textContent?.trim() === user
          )
          .map(
            (node) =>
              node.querySelector(".comment-content")?.querySelector(".comment-body")?.textContent?.trim() || ""
          );
      },
      username
    );

    currentComments.forEach((c) => comments.add(c));

    // Проверка изменения высоты страницы
    const newHeight = await page.evaluate(() => document.body.scrollHeight);
    if (newHeight === previousHeight) break;

    previousHeight = newHeight;
    attempts++;
  }

  return Array.from(comments).filter((c) => c.length > 0);
}

async function findMyComments(): Promise<CommentsResult> {
  const prUrls = readPrUrlsFromFile();
  const browser = await chromium.connectOverCDP("http://localhost:9222");
  // Используем существующую вкладку
  const contexts = browser.contexts();
  const page = contexts[0].pages()[0] || (await contexts[0].newPage());

  try {
    const results: CommentsResult = {};

    for (const prUrl of prUrls) {
      try {
        console.log(`Processing PR: ${prUrl}`);
        await page.goto(prUrl);

        // Ожидаем загрузки хотя бы одного комментария
        await page.waitForSelector(".comment", { timeout: 15000 });

        // Загружаем все комментарии
        const myComments = await loadAllComments(page, "admin");

        results[prUrl] = myComments;
      } catch (error) {
        console.error(`Error processing ${prUrl}:`);
        console.error((error as Error).message);
        results[prUrl] = [];
      }
    }

    return results;
  } finally {
    await browser.close();
  }
}

// Запуск приложения
console.log("Bitbucket PR Comments Scanner\n");
findMyComments()
  .then((results) => {
    console.log("\nРезультаты поиска комментариев:");
    for (const [url, comments] of Object.entries(results)) {
      console.log(`\nPR: ${url}`);
      console.log(`Найдено комментариев: ${comments.length}`);
      comments.forEach((comment, index) => {
        console.log(`${index + 1}. ${comment}`);
      });
    }
  })
  .catch((error) => {
    console.error("Critical error occurred:");
    console.error(error);
    process.exit(1);
  });
