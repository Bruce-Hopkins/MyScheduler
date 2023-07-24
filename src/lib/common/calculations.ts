/**
 * A function that conerts the minutes passed into the mesurement of pixels. This assures that the pixel measurements
 * are consistent no matter which component uses them.
 * @param minutes The minutes the component will represent
 * @returns The number of pixels needed
 */
export function pxPerMinute(minutes: number) {
  return minutes * 2;
}
