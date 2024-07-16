import { gql } from '@apollo/client';

export const GET_WEATHER = gql`
  query GetWeather($city: String!, $countryCode: String!) {
    weather(city: $city, countryCode: $countryCode) {
      city
      countryCode
      description
      temp
      humidity
      pressure
      windSpeed
    }
  }
`;
