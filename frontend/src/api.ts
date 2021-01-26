import { Email, Project } from "./types";
import { API } from "./api.json";
import axios from "axios";

export async function getProjects(): Promise<Array<Project>> {
  return await axios.get<Array<Project>>(`${API}/get/projects`).then((res) => {
    return res.data;
  });
}

export function sendEmail(email: Email) {}
