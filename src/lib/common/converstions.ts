export function dateToDatetimeISOString(
  dateString: string,
  hours: number,
  minutes: number
) {
  console.log("dateString is", dateString);
  const dateValues = dateString.split("-").map((value) => Number(value));
  const inputedDate = new Date(dateValues[0], dateValues[1], dateValues[2]);
  const result = inputedDate.toISOString();
  console.log("Result is", result);
  return result;
}
