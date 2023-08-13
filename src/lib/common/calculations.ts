/**
 * A function that conerts the minutes passed into the mesurement of pixels. This assures that the pixel measurements
 * are consistent no matter which component uses them.
 * @param minutes The minutes the component will represent
 * @returns The number of pixels needed
 */
export function pxPerMinute(minutes: number) {
  return minutes * 2;
}

export function dateToMinutes(date: Date) {
  const miliseconds = date.getTime();
  const seconds = miliseconds / 1000;
  const minutes = seconds / 60;
  return minutes;
}

export function addDaysToDate(date: Date, days: number): Date {
  let newDate = new Date(date);
  newDate.setDate(date.getDate() + days);

  return new Date(newDate);
}
