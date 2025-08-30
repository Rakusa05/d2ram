export type Region = "america" | "europe" | "asia" | null;

export type Account = {
  id: number;
  displayName: string;
  accountLogin: string;
  password: string;
  region: Region;
  running: boolean;
  pid: number | null;
};
