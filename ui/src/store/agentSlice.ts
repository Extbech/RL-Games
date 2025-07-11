// src/redux/slices/strengthSlice.ts
import { createApi, fetchBaseQuery } from '@reduxjs/toolkit/query/react';
import { baseURL } from '../config/urlConfig';
import type {Direction, GridBoard, TicTacAction, TicTacToeBoard } from '../types/api';

export const agentAPI = createApi({
    reducerPath: 'agentAPI',
    baseQuery: fetchBaseQuery({ baseUrl: baseURL + '/api/agent' }),
    endpoints: (builder) => ({
        getTicTacToePrediction: builder.query<TicTacAction, TicTacToeBoard>({
            query: (state) => `/predict/TicTacToe?state=${state}`,
        }),
        getGridPrediction: builder.query<Array<Array<Direction>>, GridBoard>({
            query: (state) => `/predict/Grid?state=${state}`,
    }),
    })
});

export const { useGetTicTacToePredictionQuery, useGetGridPredictionQuery } = agentAPI;
