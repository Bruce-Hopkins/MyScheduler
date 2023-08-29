export type DaySelections =
  | "Mon"
  | "Tue"
  | "Wen"
  | "Thu"
  | "Fri"
  | "Sat"
  | "Sun";

export class DaySelector {
  private _selectedDays: Map<DaySelections, boolean>;

  constructor() {
    this._selectedDays = new Map();
  }

  get daysOfTheWeek(): DaySelections[] {
    return ["Sun", "Mon", "Tue", "Wen", "Thu", "Fri", "Sat"];
  }

  get selectedDays() {
    return this._selectedDays;
  }

  addSelectedDay(day: DaySelections) {
    this._selectedDays.set(day, true);
  }

  removeSelectedDay(day: DaySelections) {
    this._selectedDays.set(day, false);
  }
}
