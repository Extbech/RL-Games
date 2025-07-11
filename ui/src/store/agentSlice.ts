// src/redux/slices/strengthSlice.ts
import { createApi, fetchBaseQuery } from '@reduxjs/toolkit/query/react';
import { baseURL } from '../config/urlConfig';
import type { TicTacToeBoard } from '../types/api';

export const agentAPI = createApi({
    reducerPath: 'agentAPI',
    baseQuery: fetchBaseQuery({ baseUrl: baseURL + '/api/agent' }),
    endpoints: (builder) => ({
        getTicTacToePrediction: builder.query<[number, number], string>({
            query: (state) => `/predict/TicTacToe?state=${state}`,
        }),
        getGridPrediction: builder.query<Array<Array<string>>, Array<Array<string>>>({
            query: (state) => `/predict/Grid?state=${state}`,
    }),
    })
});

export const { useGetTicTacToePredictionQuery, useGetGridPredictionQuery } = agentAPI;
