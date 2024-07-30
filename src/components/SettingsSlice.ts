import { createSlice } from '@reduxjs/toolkit';

const settingsSlice = createSlice({
  name: "settings",
  initialState: { month: "all" },
  reducers: {
    setMonth(state, action) {
      state.month = action.payload;
    },
  }
});

export const { setMonth } = settingsSlice.actions;
export default settingsSlice.reducer;
