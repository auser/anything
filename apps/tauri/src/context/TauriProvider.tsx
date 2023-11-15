import React, { ReactNode,useContext, useEffect, useState } from "react";

import tauriConfJson from "../../src-tauri/tauri.conf.json";
import api from '../tauri_api/api';

declare global {
  interface Window {
    __TAURI__?: any;
  }
}

export const APP_NAME = tauriConfJson.package.productName;
export const RUNNING_IN_TAURI = window.__TAURI__ !== undefined;

// NOTE: Add cacheable Tauri calls in this file
interface TauriContextInterface {
  loading: boolean;
  downloads: string;
  documents: string;
  appDocuments: string;
  osType: string;
  fileSep: string;
  currentVault: string,
}

const TauriContext = React.createContext<TauriContextInterface>({
  loading: true,
  downloads: "",
  documents: "",
  appDocuments: "",
  osType: "",
  fileSep: "/",
  currentVault: "",
});

export const useTauriContext = () => useContext(TauriContext);

export function TauriProvider({ children }: { children: ReactNode }) {
  const [loading, setLoading] = useState<boolean>(true);
  const [downloads, setDownloadDir] = useState<string>("");
  const [documents, setDocumentDir] = useState<string>("");
  const [osType, setOsType] = useState<string>("");
  const [fileSep, setFileSep] = useState<string>("/");
  const [appDocuments, setAppDocuments] = useState<string>("");
  //TODO: implement Vualts ( but after backend is done in ) 
  const [currentVault, setCurrentVault] = useState<string>("");

  //TODO: fix why this is running twice on startup
  //MIGRATE_TO_RUST
  useEffect(() => {
    if (RUNNING_IN_TAURI) {
      const callTauriAPIs = async () => {
        setDownloadDir(await api.path.downloadDir());
        const _documents = await api.path.documentDir();
        console.log("documents", _documents);
        setDocumentDir(_documents);
        const _osType = await api.os.type(); 
        setOsType(_osType);
        const _fileSep = _osType === "Windows_NT" ? "\\" : "/";
        setFileSep(_fileSep);
        setAppDocuments(`${_documents}${APP_NAME}`);
        console.log("appDocuments", `${_documents}${APP_NAME}`);
        setLoading(false);
      };
      callTauriAPIs().catch(console.error);
    }
  }, []);

  return (
    <TauriContext.Provider
      value={{ loading, fileSep, downloads, documents, osType, appDocuments, currentVault }}
    >
      {children}
    </TauriContext.Provider>
  );
}
