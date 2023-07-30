import { invoke } from "@tauri-apps/api";
import type { TaskCreate, TaskRes } from "../types/tasks";

export async function get_tasks(): Promise<TaskRes[]> {
  // Invoke the command
  return await invoke("app_get_all_tasks");
}

export async function create_task(task: TaskCreate): Promise<string> {
  const result: string = await invoke("app_create_task", { task: task });
  return result;
}

export async function delete_task(id: string): Promise<void> {
  const result: void = await invoke("app_delete_task", { id });
  return result;
}

export async function edit_task(task: TaskCreate, id: string): Promise<string> {
  const result: string = await invoke("app_delete_edit", { id, task });
  return result;
}

export async function get_task_by_id(id: string): Promise<TaskRes> {
  const result: TaskRes = await invoke("app_get_task_by_id", { id });
  return result;
}

export async function get_all_tasks(): Promise<TaskRes[][]> {
  const result: TaskRes[][] = await invoke("app_get_all_tasks");
  return result;
}
