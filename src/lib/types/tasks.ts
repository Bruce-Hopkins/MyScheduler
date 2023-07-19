export type TaskCreate = {
  body: string;
  start_at: string;
  end_at: string;
  colors: string;
};

export type TaskRes = {
  id: string;
  body: string;
  status: string;
  start_at: string;
  end_at: string;
  color: string;
  created_at: string;
};
