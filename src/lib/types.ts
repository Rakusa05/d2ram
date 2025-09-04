export type Region = "america" | "europe" | "asia" | null;
export type ConnType = "login" | "token";
export type Language = "default" | "english" | "french";

export type Account = {
  id: number;
  displayName: string;
  connectionType: ConnType;
  accountLogin: string;
  password: string;
  token: string;
  region: Region;
  running: boolean;
  pid: number | null;
};

export type AppSettings = {
  gamePath: string;
  gameLanguage: Language;
};
