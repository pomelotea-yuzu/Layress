/**
 * 静态预览的坐标换算（任务 3.3 / E07）。
 *
 * 唯一事实：矩形以「原图像素坐标」保存；显示缩放只发生在渲染那一刻。
 * 切换原始比例 / 适应区域时不得把缩放重复算进坐标，否则同一区域会漂移。
 * 这里的函数不依赖 DOM，启动时跑一组往返自检，把机器结论显示在界面上。
 */

export type DisplayMode = "original" | "fit";

export interface ImageRect {
  /** 原图像素坐标 */
  x: number;
  y: number;
  w: number;
  h: number;
}

export interface Point {
  x: number;
  y: number;
}

export interface DisplayRect {
  left: number;
  top: number;
  width: number;
  height: number;
}

/** 原始比例固定为 1；适应区域按可用区域等比缩小，不放大。 */
export function displayScale(
  mode: DisplayMode,
  imageW: number,
  imageH: number,
  areaW: number,
  areaH: number,
): number {
  if (imageW <= 0 || imageH <= 0 || areaW <= 0 || areaH <= 0) return 1;
  if (mode === "original") return 1;
  return Math.min(areaW / imageW, areaH / imageH, 1);
}

/** 显示坐标 → 原图坐标（唯一允许除以缩放的地方）。 */
export function toImagePoint(displayX: number, displayY: number, scale: number): Point {
  const s = scale > 0 ? scale : 1;
  return { x: Math.round(displayX / s), y: Math.round(displayY / s) };
}

/** 原图矩形 → 显示矩形（唯一允许乘以缩放的地方）。 */
export function toDisplayRect(rect: ImageRect, scale: number): DisplayRect {
  return {
    left: rect.x * scale,
    top: rect.y * scale,
    width: rect.w * scale,
    height: rect.h * scale,
  };
}

/** 拖拽起止点都在显示空间；结果立刻归一成原图坐标，之后与缩放无关。 */
export function rectFromDrag(start: Point, end: Point, scale: number): ImageRect {
  const a = toImagePoint(start.x, start.y, scale);
  const b = toImagePoint(end.x, end.y, scale);
  return {
    x: Math.min(a.x, b.x),
    y: Math.min(a.y, b.y),
    w: Math.abs(b.x - a.x),
    h: Math.abs(b.y - a.y),
  };
}

/**
 * 样例页的固定参照块用 CSS 像素描述（left 24 / top 24 / 160×60）。
 * DPR 会让截图像素与 CSS 像素不一致，按比例换算成原图坐标，
 * 这样两种展示模式下都能用同一块区域核对落点。
 */
export function cssRectToImage(
  css: ImageRect,
  cssViewportW: number,
  imageW: number,
): ImageRect | null {
  if (cssViewportW <= 0) return null;
  const ratio = imageW / cssViewportW;
  return {
    x: Math.round(css.x * ratio),
    y: Math.round(css.y * ratio),
    w: Math.round(css.w * ratio),
    h: Math.round(css.h * ratio),
  };
}

/** 运行时自检：显示→原图→显示往返不得累积缩放；拖拽得到的坐标也必须是原图坐标。 */
export function coordSelfTest(): { passed: boolean; cases: number; detail: string } {
  const scales = [1, 0.75, 0.5, 0.333333, 2];
  const rects: ImageRect[] = [
    { x: 0, y: 0, w: 10, h: 10 },
    { x: 24, y: 24, w: 160, h: 60 },
    { x: 317, y: 129, w: 83, h: 47 },
  ];
  let cases = 0;
  const failures: string[] = [];
  for (const scale of scales) {
    for (const rect of rects) {
      cases += 1;
      const shown = toDisplayRect(rect, scale);
      const back = toImagePoint(shown.left, shown.top, scale);
      if (Math.abs(back.x - rect.x) > 1 || Math.abs(back.y - rect.y) > 1) {
        failures.push(`scale=${scale} 往返 x/y 漂移`);
      }
      const backW = toImagePoint(shown.width, shown.height, scale);
      if (Math.abs(backW.x - rect.w) > 1 || Math.abs(backW.y - rect.h) > 1) {
        failures.push(`scale=${scale} 往返宽高漂移`);
      }
      // 同一原图矩形连续渲染两次不得被重复缩放
      const twice = toDisplayRect(
        {
          x: back.x,
          y: back.y,
          w: backW.x,
          h: backW.y,
        },
        scale,
      );
      if (Math.abs(twice.left - shown.left) > 1 || Math.abs(twice.top - shown.top) > 1) {
        failures.push(`scale=${scale} 重复计算缩放`);
      }
      cases += 1;
      const dragged = rectFromDrag(
        { x: shown.left, y: shown.top },
        { x: shown.left + shown.width, y: shown.top + shown.height },
        scale,
      );
      if (
        Math.abs(dragged.x - rect.x) > 1 ||
        Math.abs(dragged.y - rect.y) > 1 ||
        Math.abs(dragged.w - rect.w) > 1 ||
        Math.abs(dragged.h - rect.h) > 1
      ) {
        failures.push(`scale=${scale} 拖拽坐标不等于原图坐标`);
      }
    }
  }
  return {
    passed: failures.length === 0,
    cases,
    detail: failures.length === 0 ? `${cases} 组往返通过` : failures.slice(0, 3).join("；"),
  };
}
