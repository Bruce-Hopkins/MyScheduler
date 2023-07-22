type AppErrorType = "Invalid input";

class AppError extends Error {
  name: AppErrorType;
  constructor(m: string) {
    super(m);

    // Set the prototype explicitly.
    Object.setPrototypeOf(this, AppError.prototype);
  }
}
