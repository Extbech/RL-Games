import { configureStore } from '@reduxjs/toolkit';
import { persistStore, persistReducer } from 'redux-persist';
import storage from 'redux-persist/lib/storage';
import themeReducer from './themeSlice';
import { userApi } from './userSlice';
import { cardioApi } from './cardioSlice';
import { strengthApi } from './strengthSlice';
import { dietApi } from './dietSlice';


const persistConfig = {
  key: 'root',
  storage,
};

const persistedReducer = persistReducer(persistConfig, themeReducer);

export const store = configureStore({
  reducer: {
    theme: persistedReducer,
    [userApi.reducerPath]: userApi.reducer,
    [dietApi.reducerPath]: dietApi.reducer,
    [cardioApi.reducerPath]: cardioApi.reducer,
    [strengthApi.reducerPath]: strengthApi.reducer,
  },
  middleware: (getDefaultMiddleware) =>
    getDefaultMiddleware().concat(
      userApi.middleware,
      dietApi.middleware,
      cardioApi.middleware,
      strengthApi.middleware
    ),
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;

export const persistor = persistStore(store);