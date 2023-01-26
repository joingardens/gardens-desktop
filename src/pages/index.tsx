import { useQuery } from "@tanstack/react-query";
import { useEffect, useState } from "react";
import { TauriCommandService } from "../services/tauri/command.service";


function App() {

  const versionInfoQuery = useQuery({
    queryKey: [TauriCommandService.checkVersions.name],
    queryFn: () =>  TauriCommandService.checkVersions(),
  })

  return (
    <div className="h-full bg-green-200 p-6">
      <div className="h-full w-full">
          {versionInfoQuery.isLoading && "Loading"}
          {versionInfoQuery.isSuccess && 
          <div className="grid grid-cols-1 gap-3">
              <div>
              {`Docker Status: ${versionInfoQuery.data[0]}`}
              </div>
              <div>
              {`OS Status: ${versionInfoQuery.data[1]}`}
              </div>
          </div>
          }
      </div>
      <div className="h-full w-full">

      </div>
    </div>
  );
}

export default App;
