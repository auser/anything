import { useState } from "react";
import { AnythingNodeProps } from "../../utils/nodeUtils";
import { useSqlContext, EventInput } from "../../context/SqlProvider";
import { useParams } from "react-router-dom";
import clsx from "clsx";
import BaseNode from "./baseNode";
import { useFlowContext } from "../../context/FlowProvider";
import BaseNodeIcon from "../baseNodeIcon";

export default function ManualNode({ id, data }: AnythingNodeProps) {
  const { addEvent } = useSqlContext();
  const { flowFrontmatter } = useFlowContext();
  const { flow_name } = useParams();
  const [loading, setLoading] = useState(false);
  const createEvent = async () => {
    if (flow_name === undefined) return;
    if (flowFrontmatter === undefined) return;
    setLoading(true);
    let event: EventInput = {
      flow_id: flowFrontmatter.id,
      flow_name: flow_name,
      flow_version: "0.0.1",
      node_id: id,
      node_type: "manualNode", //node type for reactFlow
      node_label: "Manual Trigger", //For display in UI
      worker_name: "manual_trigger", //for accessing node results by name in props
      stage: "dev",
      worker_type: "start", //for backend processing
      event_status: "PENDING", //EVENT STATUS
      session_status: "PENDING", //SESSION STATUS
      created_at: new Date().toISOString(),
      data: "",
    };

    console.log("Adding event", event);

    addEvent(event);

    setTimeout(() => {
      setLoading(false);
    }, 1000);
  };

  return (
    <BaseNode id={id} data={data} hideIcon>
      <div className="flex flex-row">
        <button
          className={clsx(loading && "bg-green-500 rounded-md h-14 w-14")}
          onClick={() => createEvent()}
        >
          <BaseNodeIcon icon="VscPlayCircle" />
        </button>
        <div className="flex flex-col justify-center p-4">
          {data.node_label}
        </div>
      </div>
    </BaseNode>
  );
}
