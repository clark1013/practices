import {TODOApi, Configuration} from "../gen/api"

export default async function Home() {
  const config = new Configuration({
	username: process.env.TIDBCLOUD_DATA_SERVICE_PUBLIC_KEY,
	password: process.env.TIDBCLOUD_DATA_SERVICE_PRIVATE_KEY,
  });
  const api = new TODOApi(config);
  const resp = await api.getRepositories();
  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <ul className="font-mono text-xl">
        {resp.data.rows.map((repo) => (
		  <a href={repo.url}>
          <li key={repo.id}>{repo.name}</li>
		  </a>
        ))}
      </ul>

    </main>
  )
}
