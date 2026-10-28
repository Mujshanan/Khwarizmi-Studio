import { expect, test } from '@playwright/test'
import { readFile } from 'node:fs/promises'
import { resolve } from 'node:path'

const fixturePath = resolve('tests/fixtures/fidelity.md')

test('opens and exports an untouched Markdown file byte-for-byte', async ({ page }) => {
  const original = await readFile(fixturePath, 'utf8')
  await page.goto('/')
  await expect(page.locator('.milkdown .ProseMirror')).toBeVisible({ timeout: 20_000 })
  await page.locator('#file-input').setInputFiles({ name: 'fidelity.md', mimeType: 'text/markdown', buffer: Buffer.from(original, 'utf8') })
  await expect(page.locator('#document-name')).toContainText('fidelity.md')
  await expect(page.locator('#status-message')).toHaveText('Opened fidelity.md')
  await page.getByRole('button', { name: 'Source mode' }).click()
  await expect(page.locator('#source-editor')).toBeVisible()
  await expect(page.locator('#source-editor')).toHaveValue(original)
  const downloadPromise = page.waitForEvent('download')
  await page.getByRole('button', { name: 'Save .md' }).click()
  const download = await downloadPromise
  const outputPath = await download.path()
  expect(outputPath).not.toBeNull()
  expect(await readFile(outputPath!, 'utf8')).toBe(original)
})

test('reports calcpad fences without claiming to execute them', async ({ page }) => {
  await page.goto('/')
  await page.locator('#file-input').setInputFiles({ name: 'fidelity.md', mimeType: 'text/markdown', buffer: await readFile(fixturePath) })
  await expect(page.locator('#status-message')).toHaveText('Opened fidelity.md')
  await expect(page.locator('#calc-count')).toHaveText('1')
  await expect(page.getByText('Not connected')).toBeVisible()
})
