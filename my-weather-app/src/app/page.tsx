'use client';

import React, { useState } from 'react';
import { ApolloProvider, useQuery } from '@apollo/client';
import client from '../lib/apolloClient';
import { GET_WEATHER } from '../queries/weather';

const Weather = () => {
  const [city, setCity] = useState('');
  const [countryCode, setCountryCode] = useState('');

  const { loading, error, data } = useQuery(GET_WEATHER, {
    variables: { city, countryCode },
    skip: !city || !countryCode,
  });

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!city || !countryCode) return;
  };

  return (
    <div className="container mx-auto p-4">
      <h1 className="text-2xl font-bold mb-4">Weather App</h1>
      <form onSubmit={handleSubmit} className="mb-4">
        <input
          type="text"
          value={city}
          onChange={(e) => setCity(e.target.value)}
          placeholder="City"
          className="border p-2 mr-2"
        />
        <input
          type="text"
          value={countryCode}
          onChange={(e) => setCountryCode(e.target.value)}
          placeholder="Country Code"
          className="border p-2 mr-2"
        />
        <button type="submit" className="bg-blue-500 text-white p-2">Get Weather</button>
      </form>
      {loading && <p>Loading...</p>}
      {error && <p>Error: {error.message}</p>}
      {data && (
        <div className="weather-info">
          <h2 className="text-xl font-bold">{data.weather.city}, {data.weather.country_code}</h2>
          <p>{data.weather.description}</p>
          <p>Temperature: {data.weather.temp}°C</p>
          <p>Humidity: {data.weather.humidity}%</p>
          <p>Pressure: {data.weather.pressure} hPa</p>
          <p>Wind Speed: {data.weather.wind_speed} m/s</p>
        </div>
      )}
    </div>
  );
};

const Page = () => (
  <ApolloProvider client={client}>
    <Weather />
  </ApolloProvider>
);

export default Page;
