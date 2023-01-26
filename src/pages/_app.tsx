import type { AppProps } from "next/app";
import "../App.css";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";


// This default export is required in a new `pages/_app.js` file.
export default function MyApp({ Component, pageProps }: AppProps) {
  const queryClient = new QueryClient()
  return (
    <QueryClientProvider client={queryClient}>
          <Component {...pageProps} />;
    </QueryClientProvider>

  ) 
}
