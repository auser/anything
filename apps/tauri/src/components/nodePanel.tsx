import { BaseNodeIcon } from "ui";
import { useEffect, useState } from "react";
import { VscChevronDown, VscChevronUp } from "react-icons/vsc";

import { getActionNodes, getTriggerNodes } from "../utils/nodeGenerators";
import { Node, Action, Trigger } from "../utils/flowTypes";
import BaseSearch from "./baseSearch";

const NodePanel = () => {
  const [allNodes, setAllNodes] = useState<Node[]>([]);
  const [triggerNodeResults, setTriggerNodeResults] = useState<Trigger[]>([]);
  const [actionNodeResults, setActionNodeResults] = useState<Action[]>([]);
  const [showActions, setShowActions] = useState(true);
  const [showTriggers, setShowTriggers] = useState(true);

  useEffect(() => {
    console.log("Initial hydrate");
    let action_nodes = getActionNodes();
    let trigger_nodes = getTriggerNodes();

    // populate original data to maintain for search
    setAllNodes([...action_nodes, ...trigger_nodes]);

    //popoulate results as all data to begin
    setActionNodeResults(action_nodes);
    setTriggerNodeResults(trigger_nodes);
  }, []);

  const setResults = (results: Node[]) => {
    setTriggerNodeResults(
      results.filter((node): node is Trigger => node.trigger)
    );
    setActionNodeResults(
      results.filter((node): node is Action => !node.trigger)
    );
  };

  return (
    <div className="hide-scrollbar max-h-screen overflow-y-auto p-4">
      <div className="py-4">
        <BaseSearch
          data={allNodes}
          searchKey={["node_label"]}
          onResultsChange={(results) => {
            setResults(results);
          }}
        />
      </div>

      <h1
        onClick={() => setShowTriggers(!showTriggers)}
        className="flex h-12 cursor-pointer flex-row justify-between py-2 pb-2 text-xl font-bold"
      >
        Triggers
        {showTriggers ? <VscChevronDown /> : <VscChevronUp />}
      </h1>
      <div
        className={`transition-max-height overflow-hidden pb-2 duration-500 ease-in-out ${
          showTriggers ? "max-h-auto" : "max-h-0"
        }`}
      >
        {triggerNodeResults.map((node: Trigger) => (
          <NodeDnD node={node} key={node.node_label} />
        ))}
      </div>
      <h1
        onClick={() => setShowActions(!showActions)}
        className="flex cursor-pointer flex-row justify-between py-2 pb-2 text-xl font-bold"
      >
        Actions
        {showActions ? <VscChevronDown /> : <VscChevronUp />}
      </h1>
      <div
        className={`transition-max-height overflow-hidden pb-2 duration-500 ease-in-out ${
          showActions ? "max-h-auto" : "max-h-0"
        }`}
      >
        {actionNodeResults.map((node: Action) => (
          <NodeDnD node={node} key={node.node_label} />
        ))}
      </div>
    </div>
  );
};

const NodeDnD = ({ node }: { node: Node }) => {
  const onDragStart = (event: any) => {
    // let nodeType;

    // if ('trigger' in node && node.trigger !== true) {
    //   nodeType = "superNode";
    // } else {
    //   nodeType = node.nodePresentationData.nodeType;
    // }

    // console.log("drag started", nodeType);

    // event.dataTransfer.setData("nodeType", nodeType);
    event.dataTransfer.setData("nodeData", JSON.stringify(node));
    // event.dataTransfer.setData(
    //   "nodeConfigurationData",
    //   JSON.stringify(node.nodeConfigurationData)
    // );
    // event.dataTransfer.setData(
    //   "nodePresentationData",
    //   JSON.stringify(node.nodePresentationData)
    // );
    event.dataTransfer.effectAllowed = "move";
  };

  return (
    <div
      className="mt-2 flex max-w-md cursor-grab flex-row items-center rounded-md bg-white bg-opacity-5 p-2 pb-2"
      onDragStart={(event) => onDragStart(event)}
      draggable
    >
      <BaseNodeIcon
        icon={node.icon}
        className={`h-9 w-9 bg-opacity-80 text-white`}
      />
      <h1 className="truncate overflow-ellipsis pl-2 text-lg">
        {node.node_label}
      </h1>
    </div>
  );
};

export default NodePanel;
