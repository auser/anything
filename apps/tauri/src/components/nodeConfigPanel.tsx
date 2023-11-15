import { useEffect, useState } from "react";
import { Controller, useForm } from "react-hook-form";
import { useParams } from "react-router-dom";

import { useFlowNavigationContext } from "../context/FlowNavigationProvider";
import { useFlowContext } from "../context/FlowProvider";

const NodeConfigPanel = () => { 
  const { nodeId, setNodeConfigPanel } = useFlowNavigationContext();
  const { readNodeConfig, writeNodeConfig } = useFlowContext();

  const [data, setData] = useState<Node | undefined>();

  const {
    register,
    handleSubmit,
    setValue,
    control,
    formState: { errors },
  } = useForm();

  const hydrate = async () => {
    try {
   
      if (!nodeId) return;

      //Get Node Configuration
      const res: any = await readNodeConfig(nodeId);

      console.log("res in nodeConfig", res);

      if (res === undefined) return;
      //set keys on form
      Object.keys(res).forEach((key) => {
        setValue(key, res[key]);
      });

      //set all data for display
      setData(res);
    } catch (error) {
      console.log("error", error);
    }
  };

  useEffect(() => {
    hydrate();
  }, []);

  const onSubmit = (data: any) => {
    if (!nodeId) return;
    console.log("Hit Node Config Submit");
    console.log(data);

    writeNodeConfig(nodeId, data);
    setNodeConfigPanel(false, "");
  };

  return (
    <div className="flex flex-col h-full overflow-y-auto p-4">
      <h1 className="text-2xl font-bold">Node Settings</h1>
      {nodeId ? (
        <form
          className="flex flex-col gap-4 p-4"
          onSubmit={handleSubmit(onSubmit)}
        >
          {data
            ? Object.keys(data).map((key, index) => {
                const value = data[key];

                if (typeof value === "string" || typeof value === "number") {
                  return (
                    <div key={key}>
                      <div className="mb-1">{key}:</div>
                      <input
                        type="text"
                        className="input input-bordered input-md w-full"
                        defaultValue={value}
                        {...register(key)}
                      />
                      {errors[key] && (
                        <span>{JSON.stringify(errors[key]?.message)}</span>
                      )}
                    </div>
                  );
                } else if (typeof value === "boolean") {
                  return (
                    <Controller
                      key={index}
                      name={key}
                      control={control}
                      defaultValue={value}
                      render={({ field }) => (
                        <label>
                          {key}:
                          <input
                            className="toggle toggle-success"
                            type="checkbox"
                            {...field}
                            checked={field.value}
                          />
                        </label>
                      )}
                    />
                  );
                } else {
                  return null;
                }
              })
            : null}
          {/* //TODO other data types */}
          <button className="mt-2 btn btn-primary" type="submit">
            Save
          </button>
        </form>
      ) : (
        <div>Select a node to configure</div>
      )}
    </div>
  );
};

export default NodeConfigPanel;
