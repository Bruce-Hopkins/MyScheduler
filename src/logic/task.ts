import { pxPerMinute } from "../lib/common/calculations";
import type { TaskRes } from "../lib/types/tasks";

type Color = `#${string}`;

export class Task {
  private task: TaskRes;

  public start_at: Date;
  public end_at: Date;
  public color: Color;

  constructor(task: TaskRes) {
    try {
      console.log("task is", task);
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

    console.log(this.color);
  }

  get minutes() {
    const miliseconds = this.end_at.getTime() - this.start_at.getTime();
    console.log("Miliseconds are", miliseconds);
    const seconds = miliseconds / 1000;
    const minutes = seconds / 60;
    console.log("Minutes are", minutes);
    return minutes;
  }

  get height() {
    const height = pxPerMinute(this.minutes);
    console.log("height is", height);
    return height;
  }

  get body() {
    return this.task.body;
  }

  get status() {
    return this.task.status;
  }
}
