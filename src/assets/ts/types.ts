export interface Response<T> {
  success: boolean;
  body?: T;
}

export interface Secret {
  label: string;
  kind: string;
  data: string;
}
