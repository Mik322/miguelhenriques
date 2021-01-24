import { useState, useEffect } from "react";
import { useParams } from "react-router-dom";
import { Project, ProjectPath } from "../types";
import { getProjectById } from "../api";
import { useImageContainer } from "../hooks";

export default function ProjectPage() {
  let { projectID } = useParams<ProjectPath>();
  let [project, setProject] = useState<Project | null>(null);
  let imageContainer = useImageContainer(project);

  useEffect(() => {
    const id = parseInt(projectID);
    if (!isNaN(id)) {
      getProjectById(id)
        .then((prj) => setProject(prj))
        .catch(() => setProject(null));
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  if (project === null) return <p>Project does not exist</p>;

  return (
    <div>
      <h2>{project.name}</h2>
      {imageContainer}
      <p>{project.description}</p>
    </div>
  );
}
