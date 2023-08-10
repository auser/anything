import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { useSqlContext } from "../context/SqlProvider";

export default function Tables() {
  const { getTableData } = useSqlContext();
  const [data, setData] = useState<any[]>([]);

  const { table } = useParams();

  const hydrate = async () => {
    try {
      if (!table) return;
      const data = await getTableData(table);
      console.log("data in tableData", data);
      setData(data);
    } catch (error) {
      console.log("error", error);
    }
  };

  useEffect(() => {
    hydrate();
  }, []);
  return (
    <div className="flex flex-col h-full w-full m-10">
      <div className="text-5xl text-white m-5">table/{table}</div>

      <table className="table table-xs">
        <thead>
          <tr>
            <th>ID</th>
            <th>Created At</th>
            <th>Flow ID</th>
            <th>Flow Name</th>
            <th>Flow Version</th>
            <th>Stage</th>
            <th>Status</th>
            <th>Data</th>
          </tr>
        </thead>
        <tbody>
          {data.map((event: any) => {
            return (
              <tr>
                <th>{event.event_id}</th>
                <th>{event.created_at}</th>
                <th>{event.flow_id}</th>
                <th>{event.flow_name}</th>
                <th>{event.flow_version}</th>
                <th>{event.stage}</th>
                <th>{event.status}</th>
                <th>{JSON.stringify(event.data)}</th>
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
}