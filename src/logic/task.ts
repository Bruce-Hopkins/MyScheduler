import {
  dateToMinutes,
  getMinutesString,
  pxPerMinute,
} from "../lib/common/calculations";
import type { Color } from "../lib/types/common";
import type { TaskRes } from "../lib/types/tasks";

export class TaskModel {
  private task: TaskRes;

  public start_at: Date;
  public end_at: Date;
  public color: Color;

  constructor(task: TaskRes) {
    try {
      this.start_at = new Date(task.start_at);
      this.end_at = new Date(task.end_at);
    } catch (e) {
      let error = new AppError("Cannot format task dates");
      error.name = "Invalid input";
      throw error;
    }

    if (task.color.charAt(0) !== "#") {
      let error = new AppError("Invalid color for the task");
      error.name = "Invalid input";
    }

    this.color = task.color as Color;
    this.task = task;
  }

  get top(): number {
    const today = new Date(this.start_at);

    today.setHours(0);
    today.setMinutes(0);

    const todayMinutes = dateToMinutes(today);
    const startDateMinutes = dateToMinutes(this.start_at);
    return pxPerMinute(startDateMinutes) - pxPerMinute(todayMinutes);
  }

  get minutes() {
    const miliseconds = this.end_at.getTime() - this.start_at.getTime();
    const seconds = miliseconds / 1000;
    const minutes = seconds / 60;
    return minutes;
  }

  get time() {
    const minutes = getMinutesString(this.start_at);
    return `${this.start_at.getHours()}:${minutes}`;
  }

  get date() {
    const months = [
      "January",
      "February",
      "March",
      "April",
      "May",
      "June",
      "July",
      "August",
      "September",
      "October",
      "November",
      "December",
    ];

    const monthName = months[this.start_at.getMonth()];
    return `${monthName} ${this.start_at.getDate()}`;
  }

  get height() {
    const height = pxPerMinute(this.minutes);
    return height;
  }

  get body() {
    return this.task.body;
  }

  get status() {
    return this.task.status;
  }
}
