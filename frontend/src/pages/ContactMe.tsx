import React, { useState } from "react";
import { sendEmail } from "../api";
import { Email } from "../types";
import "../static/ContactMe.css";

export default function ContactMe() {
  const [name, setName] = useState("");
  const [from, setFrom] = useState("");
  const [subject, setSubject] = useState("");
  const [content, setContent] = useState("");

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const email: Email = {
      name: name,
      from: from,
      subject: subject,
      text: content,
    };
    sendEmail(email);
  };

  return (
    <div className="contact-me">
      <h1>Contact Me</h1>
      <form onSubmit={handleSubmit}>
        <label>
          Name:
          <input
            name="name"
            type="text"
            value={name}
            onChange={(e) => setName(e.target.value)}
          />
        </label>
        <label>
          Email:
          <input
            type="email"
            value={from}
            onChange={(e) => setFrom(e.target.value)}
          />
        </label>
        <label>
          Subject:
          <input
            type="text"
            value={subject}
            onChange={(e) => setSubject(e.target.value)}
          />
        </label>
        <label>
          Content:
          <textarea
            value={content}
            onChange={(e) => setContent(e.target.value)}
          />
        </label>
        <input type="submit" value="Submit" />
      </form>
    </div>
  );
}
