import { useQuery } from "@tanstack/react-query";
import { useEffect, useState } from "react";
import { TauriCommandService } from "../services/tauri/command.service";


function App() {

  const versionInfoQuery = useQuery({
    queryKey: [TauriCommandService.checkVersions.name],
    queryFn: () =>  TauriCommandService.checkVersions(),
    cacheTime: Infinity,
    refetchOnMount: false,
    refetchOnWindowFocus: false,
  })

  return (
    <div className="h-full bg-green-200 relative box-border">
      <div className="h-full w-full p-6">
          {versionInfoQuery.isLoading && "Loading"}
          {versionInfoQuery.isSuccess && 
          <div className="grid grid-cols-1 gap-2">
              <div>
              {`Docker Status: ${versionInfoQuery.data[0]}`}
              </div>
              <div>
              {`OS Status: ${versionInfoQuery.data[1]}`}
              </div>
              <div>
              {`Root status: ${versionInfoQuery.data[2]}`}
              </div>
          </div>
          }
          {versionInfoQuery.isSuccess &&
            <button 
            className={`bg-blue-400 p-2 mt-6 cursor-pointer hover:bg-blue-500 transition-all rounded`} 
            disabled={!versionInfoQuery.data.includes("RED")}>
              Install Gardens
            </button>
          }
      </div>
      <div className="h-full w-full">

      </div>
    </div>
  );
}

export default App;
