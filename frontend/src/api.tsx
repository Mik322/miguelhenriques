import { Email, Project } from "./types";
import { API } from "./api.json";
import axios from "axios";

export async function getProjects(): Promise<Array<Project>> {
  return await axios.get<Array<Project>>(`${API}/get/projects`).then((res) => {
    return res.data;
  });
}

export async function getProjectById(id: number): Promise<Project | null> {
  const project = await axios
    .get<Project>(`${API}/get/project/${id}`)
    .then((res) => {
      return res.data;
    })
    .catch(() => {
      return null;
    });

  return project;
}

export function sendEmail(email: Email) {}
