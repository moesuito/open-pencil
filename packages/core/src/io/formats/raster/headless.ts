import type { CanvasKit } from 'canvaskit-wasm'

import type { SceneGraph } from '@open-pencil/scene-graph'

import { getCanvasKit } from '#core/canvaskit'
import { SkiaRenderer } from '#core/canvas'
import { IS_BROWSER } from '#core/constants'

import { renderNodesToImage, renderThumbnail, type ExportFormat } from './render'

let cachedCk: CanvasKit | null = null
let cachedRenderer: SkiaRenderer | null = null

/**
 * `new URL(...).pathname` leaves a leading slash before the drive letter on Windows
 * (`/C:/…`), which no filesystem call accepts. CanvasKit's `locateFile` needs a real path.
 */
function fileUrlPath(url: URL): string {
  const path = decodeURIComponent(url.pathname)
  return /^\/[A-Za-z]:/.test(path) ? path.slice(1) : path
}

/**
 * Desktop runs resolve the full CanvasKit build from disk; in the editor's own webview the
 * regular build is already bundled and served, so it is used instead.
 */
async function createCanvasKit(): Promise<CanvasKit> {
  if (IS_BROWSER) return getCanvasKit()
  const CanvasKitInit = (await import('canvaskit-wasm/full')).default
  const ckPath = import.meta.resolve('canvaskit-wasm/full')
  const binDir = fileUrlPath(new URL('.', ckPath))
  return CanvasKitInit({ locateFile: (file: string) => binDir + file })
}

export async function initCanvasKit(): Promise<CanvasKit> {
  if (cachedCk) return cachedCk
  cachedCk = await createCanvasKit()
  return cachedCk
}

async function getRenderer(): Promise<{ ck: CanvasKit; renderer: SkiaRenderer }> {
  const ck = await initCanvasKit()
  if (cachedRenderer) return { ck, renderer: cachedRenderer }
  const surface = ck.MakeSurface(1, 1)
  if (!surface) throw new Error('Failed to create CanvasKit surface')
  const renderer = new SkiaRenderer(ck, surface)
  renderer.viewportWidth = 1
  renderer.viewportHeight = 1
  renderer.dpr = 1
  await renderer.loadFonts()
  cachedRenderer = renderer
  return { ck, renderer }
}

export async function headlessRenderNodes(
  graph: SceneGraph,
  pageId: string,
  nodeIds: string[],
  options: {
    scale?: number
    format?: ExportFormat
    quality?: number
    trimTransparent?: boolean
  } = {}
): Promise<Uint8Array | null> {
  const { ck, renderer } = await getRenderer()
  renderer.invalidateAllPictures()
  await renderer.prepareForExport(graph, pageId, nodeIds)
  return renderNodesToImage(ck, renderer, graph, pageId, nodeIds, {
    scale: options.scale ?? 1,
    format: options.format ?? 'PNG',
    quality: options.quality,
    trimTransparent: options.trimTransparent
  })
}

export async function headlessRenderThumbnail(
  graph: SceneGraph,
  pageId: string,
  width: number,
  height: number
): Promise<Uint8Array | null> {
  const { ck, renderer } = await getRenderer()
  renderer.invalidateAllPictures()
  return renderThumbnail(ck, renderer, graph, pageId, width, height)
}
