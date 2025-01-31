import { configureStore } from "@reduxjs/toolkit";
import settingsReducer from "./components/SettingsSlice";

export const store = configureStore({
  reducer: {
    settings: settingsReducer,
  }
});

export type RootState = ReturnType<typeof store.getState>;
