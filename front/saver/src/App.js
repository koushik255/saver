import React, { useState, useEffect } from "react";
import axios from "axios";
import "./App.css";

const API_BASE_URL = "http://localhost:8080";

function App() {
  const [people, setPeople] = useState([]);
  const [peopleText, setPeopleText] = useState("");
  const [newPerson, setNewPerson] = useState({ name: "", post: "" });
  const [searchTerm, setSearchTerm] = useState("");
  const [searchType, setSearchType] = useState("name");
  const [searchResults, setSearchResults] = useState([]);
  const [loading, setLoading] = useState(false);
  const [viewMode, setViewMode] = useState("json");

  // Fetch all people as JSON (/blud)
  const fetchPeopleJSON = async () => {
    try {
      setLoading(true);
      const response = await axios.get(`${API_BASE_URL}/blud`);
      setPeople(response.data);
    } catch (error) {
      console.error("Error fetching people JSON:", error);
    } finally {
      setLoading(false);
    }
  };

  // Fetch all people as text (/list)
  const fetchPeopleText = async () => {
    try {
      setLoading(true);
      const response = await axios.get(`${API_BASE_URL}/list`);
      setPeopleText(response.data);
    } catch (error) {
      console.error("Error fetching people text:", error);
    } finally {
      setLoading(false);
    }
  };

  // Add new person
  const addPerson = async (e) => {
    e.preventDefault();
    if (!newPerson.name || !newPerson.post) return;

    try {
      setLoading(true);
      await axios.get(
        `${API_BASE_URL}/add/${newPerson.name}|${newPerson.post}`
      );
      setNewPerson({ name: "", post: "" });
      // Refresh current view
      if (viewMode === "json") {
        fetchPeopleJSON();
      } else {
        fetchPeopleText();
      }
    } catch (error) {
      console.error("Error adding person:", error);
    } finally {
      setLoading(false);
    }
  };

  // Search people
  const searchPeople = async () => {
    if (!searchTerm) return;

    try {
      setLoading(true);
      const endpoint =
        searchType === "name"
          ? `${API_BASE_URL}/find/${searchTerm}`
          : `${API_BASE_URL}/search/${searchTerm}`;
      const response = await axios.get(endpoint);
      setSearchResults(response.data);
    } catch (error) {
      console.error("Error searching people:", error);
    } finally {
      setLoading(false);
    }
  };

  // Handle view mode change
  const handleViewModeChange = (mode) => {
    setViewMode(mode);
    if (mode === "json") {
      fetchPeopleJSON();
    } else {
      fetchPeopleText();
    }
  };

  useEffect(() => {
    fetchPeopleJSON(); // Default to JSON view
  }, []);

  return (
    <div className="App">
      <header className="App-header">
        <h1>Person Manager</h1>
      </header>

      <main className="container">
        {/* Add Person Form */}
        <section className="add-person">
          <h2>Add New Person</h2>
          <form onSubmit={addPerson}>
            <input
              type="text"
              placeholder="Name"
              value={newPerson.name}
              onChange={(e) =>
                setNewPerson({ ...newPerson, name: e.target.value })
              }
              required
            />
            <input
              type="text"
              placeholder="Post"
              value={newPerson.post}
              onChange={(e) =>
                setNewPerson({ ...newPerson, post: e.target.value })
              }
              required
            />
            <button type="submit" disabled={loading}>
              {loading ? "Adding..." : "Add Person"}
            </button>
          </form>
        </section>

        {/* Search Section */}
        <section className="search">
          <h2>Search People</h2>
          <input
            type="text"
            placeholder="Search term"
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
          />
          <select
            value={searchType}
            onChange={(e) => setSearchType(e.target.value)}
          >
            <option value="name">Search by Name (/find)</option>
            <option value="post">Search by Post (/search)</option>
          </select>
          <button onClick={searchPeople} disabled={loading}>
            Search
          </button>

          {searchResults.length > 0 && (
            <div className="search-results">
              <h3>Search Results:</h3>
              {searchResults.map((person, index) => (
                <div key={index} className="person-card">
                  <h4>{person.name}</h4>
                  <p>Post: {person.post}</p>
                  <p>Year of Birth: {person.yob}</p>
                  <p>ID: {person.id}</p>
                </div>
              ))}
            </div>
          )}
        </section>

        {/* All People List */}
        <section className="people-list">
          <div className="list-header">
            <h2>All People</h2>
            <div className="view-controls">
              <button
                className={viewMode === "json" ? "active" : ""}
                onClick={() => handleViewModeChange("json")}
              >
                JSON View (/blud)
              </button>
              <button
                className={viewMode === "text" ? "active" : ""}
                onClick={() => handleViewModeChange("text")}
              >
                Text View (/list)
              </button>
            </div>
          </div>

          <button
            onClick={viewMode === "json" ? fetchPeopleJSON : fetchPeopleText}
            disabled={loading}
          >
            {loading ? "Loading..." : "Refresh"}
          </button>

          {loading ? (
            <p>Loading...</p>
          ) : viewMode === "json" ? (
            <div className="people-grid">
              {people.map((person, index) => (
                <div key={index} className="person-card">
                  <h4>{person.name}</h4>
                  <p>Post: {person.post}</p>
                  <p>Year of Birth: {person.yob}</p>
                  <p>ID: {person.id}</p>
                </div>
              ))}
            </div>
          ) : (
            <div className="text-view">
              <pre>{peopleText || "No people found."}</pre>
            </div>
          )}
        </section>
      </main>
    </div>
  );
}

export default App; 