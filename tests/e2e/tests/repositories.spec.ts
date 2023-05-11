import { test, expect } from "@playwright/test";

test("show repositories on dashboard", async ({ page }) => {
  await page.goto("/");

  const cards = await page.getByTestId("card");
  await expect(cards).toHaveCount(8);

  const firstCard = cards.filter({
    has: page.getByText("EXP/example-frontend"),
  });
  await expect(firstCard).toBeVisible();

  await expect(firstCard.getByText("main")).toBeVisible();
  await expect(firstCard.getByText("feature/matrix-index")).toBeVisible();
});
