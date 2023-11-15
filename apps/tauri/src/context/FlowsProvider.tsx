import {
  createContext,
  ReactNode,
  useContext,
  useEffect,
  useState,
} from "react";

import api from "../tauri_api/api";
import { Flow } from "../utils/flowTypes";
import { UpdateFlowArgs } from "tauri-plugin-anything-tauri/webview-src";

interface FlowsContextInterface {
  flows: Flow[];
  createNewFlow: () => void;
  deleteFlow: (flowName: string) => void;
  updateFlow: (flowId: string, args: UpdateFlowArgs) => void;
}

export const FlowsContext = createContext<FlowsContextInterface>({
  flows: [],
  createNewFlow: () => {},
  deleteFlow: () => {},
  updateFlow: () => {},
});

export const useFlowsContext = () => useContext(FlowsContext);

export const FlowsProvider = ({ children }: { children: ReactNode }) => {

  const [flows, setFlows] = useState<Flow[]>([]);

  //BUG: there is a bug where when you add new flows the names colide because we write files as names.
  //To reproduce. Create 2 flows. they will be called "Flow 1 and Flow 2"
  //Delete Flow 1. Now Create a new Flow and it will try to create "Flow 2" again and fail.
  //TODO: more sophisticated way of determining new flow name
  const createNewFlow = async (): Promise<any> => {
    try {
      //TODO Move to DB to fix collision problem
      let flowName = "Flow" + " " + (flows.length + 1);
      console.log("Creating new Flow in FlowsProvider");
      await api.flows.createFlow(flowName);
    } catch (error) {
      console.log("error creating new flow in FlowsProvider", error);
      console.error(error);
    } finally {
      getFlows();
    }
  };

  const deleteFlow = async (flowName: string): Promise<any> => {
    //TODO: deal with situation where there are flow events in the db
    try {
      await api.flows.deleteFlow(flowName);
    } catch (error) {
      console.error(error);
    } finally {
      getFlows();
    }
  };

  const updateFlow = async (flowId: string, args: UpdateFlowArgs) => {
    //Update Flow
    let res = await api.flows.updateFlow(flowId, args);
    //Rehydrate Global State
    getFlows();
    //Return without waiting
    return res;
  };

  const getFlows = async () => {
    console.log("Getting Flows from Tauri API?");
    let res: any = await api.flows.getFlows();
    console.log("Got New Flows", res);
    setFlows(res.flows);
  };

  //Hydrate flows on launch
  useEffect(() => {
    getFlows();
  }, []);

  return (
    <FlowsContext.Provider
      value={{
        flows,
        createNewFlow,
        deleteFlow,
        updateFlow,
      }}
    >
      {children}
    </FlowsContext.Provider>
  );
};
