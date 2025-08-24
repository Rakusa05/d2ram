export type Account = {
  id: number;
  displayName: string;
  accountLogin: string;
  password: string;
  region: "america" | "europe" | "asia" | null;
  running: boolean;
  pid: number | null;
};
