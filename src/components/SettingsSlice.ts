import { createSlice } from '@reduxjs/toolkit';

const settingsSlice = createSlice({
  name: "settings",
  initialState: { month: "all", darkMode: false },
  reducers: {
    setMonth(state, action) {
      state.month = action.payload;
    },
    setDarkMode(state, action) {
      state.darkMode = action.payload;
    }
  }
});

export const { setMonth, setDarkMode } = settingsSlice.actions;
export default settingsSlice.reducer;
