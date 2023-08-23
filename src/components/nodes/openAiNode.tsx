import React, { useCallback } from "react";
import ReactFlow, { Handle, Position } from "reactflow";
import { Node } from "../nodePanel";

let node: Node = {
  nodeType: "openAiNode",
  title: "OpenAI Node",
  alt: "OpenAI Node",
  nodeData: {
    worker_type: "rest",
  },
  specialData: {
    url: "",
  },
};

OpenAiNode.Node = node;

export default function OpenAiNode({ data }: { data: any }) {
  return (
    <div
      className={
        "bg-primary w-40 h-20 p-4 border rounded-md text-primary-content flex flex-col justify-center align-middle" +
        data.classNames
      }
    >
      <Handle type="target" position={Position.Top} id="a" />
      <div className="text-left text-xl">Open AI Node</div>
      <Handle type="source" position={Position.Bottom} id="b" />
    </div>
  );
}
