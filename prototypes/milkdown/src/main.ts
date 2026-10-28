import { Crepe } from '@milkdown/crepe'
import '@milkdown/crepe/theme/common/style.css'
import '@milkdown/crepe/theme/crepe-dark.css'
import './style.css'

const starterMarkdown = `# Khwarizmi Studio

*A Markdown-first workspace for engineering calculations.*

Write technical notes, document assumptions, and keep equations beside the engineering narrative.

## Example: simply supported beam

For a uniformly distributed load, the maximum bending moment is:

$$
M_{max} = \\frac{wL^2}{8}
$$

The following block is detected as a calculation region, but **is not executed** in this prototype.

\`\`\`calcpad
L = 6
w = 12
M = w*L^2/8
\`\`\`

> Design note: confirm units, load combinations, and the governing code before using any calculation in practice.

## Working notes

- Keep assumptions explicit.
- Use SI units consistently.
- Validate results against an independent example.
`

const editorHost = document.querySelector<HTMLDivElement>('#editor')!
const sourceEditor = document.querySelector<HTMLTextAreaElement>('#source-editor')!
const openButton = document.querySelector<HTMLButtonElement>('#open-button')!
const sourceButton = document.querySelector<HTMLButtonElement>('#source-button')!
const saveButton = document.querySelector<HTMLButtonElement>('#save-button')!
const fileInput = document.querySelector<HTMLInputElement>('#file-input')!
const documentName = document.querySelector<HTMLDivElement>('#document-name')!
const statusMessage = document.querySelector<HTMLSpanElement>('#status-message')!
const documentStats = document.querySelector<HTMLSpanElement>('#document-stats')!
const calcCount = document.querySelector<HTMLDivElement>('#calc-count')!
const modeLabel = document.querySelector<HTMLSpanElement>('#mode-label')!

let crepe: Crepe | null = null
let sourceMode = false
let currentFileName = 'Untitled.md'
let markdownCache = starterMarkdown
let dirty = false
let loading = false

function setStatus(message: string) {
  statusMessage.textContent = message
}

function setDirty(value: boolean) {
  dirty = value
  const dot = document.querySelector<HTMLSpanElement>('.saved-dot')
  if (dot) {
    dot.classList.toggle('unsaved', dirty)
    dot.title = dirty ? 'Unsaved changes' : 'Saved in current session'
  }
}

function getMarkdown(): string {
  return sourceMode ? sourceEditor.value : (crepe?.getMarkdown() ?? markdownCache)
}

function refreshStats(markdown = getMarkdown()) {
  const words = markdown.trim().match(/\S+/g)?.length ?? 0
  const blocks = [...markdown.matchAll(/^\s*`{3,}calcpad\s*$/gim)].length
  documentStats.textContent = `${words} words · ${blocks} calculation block${blocks === 1 ? '' : 's'}`
  calcCount.textContent = String(blocks)
}

function setDocumentName(name: string) {
  currentFileName = name
  documentName.textContent = name
  const dot = document.createElement('span')
  dot.className = 'saved-dot'
  dot.title = 'Saved in current session'
  documentName.append(' ', dot)
}

async function mountEditor(markdown: string) {
  loading = true
  if (crepe) {
    await crepe.destroy()
    crepe = null
  }
  editorHost.replaceChildren()
  crepe = new Crepe({
    root: editorHost,
    defaultValue: markdown,
    features: {
      [Crepe.Feature.Latex]: true,
      [Crepe.Feature.TopBar]: true,
    },
    featureConfigs: {
      [Crepe.Feature.Placeholder]: {
        text: 'Start your engineering notes…',
        mode: 'block',
      },
      [Crepe.Feature.Latex]: {
        katexOptions: { throwOnError: false },
      },
    },
  })
  await crepe.create()
  crepe.on((listener) => {
    listener.markdownUpdated((_ctx, value) => {
      markdownCache = value
      if (!loading) setDirty(true)
      refreshStats(value)
    })
  })
  loading = false
  markdownCache = crepe.getMarkdown()
  refreshStats(markdownCache)
}

function enterSourceMode() {
  markdownCache = getMarkdown()
  sourceMode = true
  sourceEditor.value = markdownCache
  sourceEditor.hidden = false
  editorHost.hidden = true
  sourceButton.textContent = 'Visual mode'
  modeLabel.textContent = 'RAW MARKDOWN SOURCE'
  setStatus('Source mode · edits are applied when returning to visual mode')
  refreshStats(markdownCache)
  sourceEditor.focus()
}

async function enterVisualMode() {
  markdownCache = sourceEditor.value
  sourceMode = false
  sourceEditor.hidden = true
  editorHost.hidden = false
  sourceButton.disabled = true
  try {
    await mountEditor(markdownCache)
    sourceButton.textContent = 'Source mode'
    modeLabel.textContent = 'VISUAL MARKDOWN'
    setStatus('Visual editor ready')
  } catch (error) {
    sourceMode = true
    sourceEditor.hidden = false
    editorHost.hidden = true
    setStatus(`Could not render document: ${String(error)}`)
  } finally {
    sourceButton.disabled = false
    refreshStats(markdownCache)
  }
}

openButton.addEventListener('click', () => fileInput.click())
fileInput.addEventListener('change', async () => {
  const file = fileInput.files?.[0]
  if (!file) return
  const content = await file.text()
  markdownCache = content
  setDocumentName(file.name)
  setDirty(false)
  if (sourceMode) {
    sourceEditor.value = content
    refreshStats(content)
  } else {
    await mountEditor(content)
  }
  setStatus(`Opened ${file.name}`)
  fileInput.value = ''
})

sourceButton.addEventListener('click', async () => {
  if (sourceMode) await enterVisualMode()
  else enterSourceMode()
})

sourceEditor.addEventListener('input', () => {
  markdownCache = sourceEditor.value
  setDirty(true)
  refreshStats(markdownCache)
})

saveButton.addEventListener('click', () => {
  const markdown = getMarkdown()
  const blob = new Blob([markdown], { type: 'text/markdown;charset=utf-8' })
  const url = URL.createObjectURL(blob)
  const link = document.createElement('a')
  link.href = url
  link.download = currentFileName.toLowerCase().endsWith('.md') ? currentFileName : `${currentFileName}.md`
  document.body.append(link)
  link.click()
  link.remove()
  URL.revokeObjectURL(url)
  markdownCache = markdown
  setDirty(false)
  setStatus(`Exported ${link.download}`)
})

window.addEventListener('beforeunload', (event) => {
  if (!dirty) return
  event.preventDefault()
  event.returnValue = ''
})

await mountEditor(starterMarkdown)
setDocumentName(currentFileName)
setDirty(false)
setStatus('Ready · Markdown source is preserved on export')
