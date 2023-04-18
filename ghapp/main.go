package main

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"os"

	"github.com/bradleyfalzon/ghinstallation/v2"
	"github.com/google/go-github/v50/github"
	"github.com/sergi/go-diff/diffmatchpatch"
	"golang.org/x/oauth2"
)

func makeCommits() {
	commitAsGithubApp()
	commitOnBehalfOfUser()
}

func Str2Ptr(t string) *string {
	return &t
}

func commitAsGithubApp() {
	// Shared transport to reuse TCP connections.
	tr := http.DefaultTransport
	// Wrap the shared transport for use with the app ID 1 authenticating with installation ID 99.
	// itr, err := ghinstallation.NewKeyFromFile(tr, 315666, 36256853, "/Users/wenjiazhi/playground/wenjiazhi.2023-04-09.private-key.pem")
	itr, err := ghinstallation.NewKeyFromFile(tr, 315666, 36392445, "/Users/wenjiazhi/playground/wenjiazhi.2023-04-09.private-key.pem")
	if err != nil {
		log.Fatal(err)
	}
	// Use installation transport with github.com/google/go-github
	client := github.NewClient(&http.Client{Transport: itr})
	ctx := context.Background()
	// a, b, _, err := client.Repositories.GetContents(ctx, "clark1013", "activity_demo", "/", &github.RepositoryContentGetOptions{})
	fileContent, _, _, err := client.Repositories.GetContents(ctx, "clarkdgh", "just4test", "github_app.txt", &github.RepositoryContentGetOptions{})
	if err != nil {
		panic(err)
	}
	// Create File as Github App
	c, _, err := client.Repositories.CreateFile(
		ctx,
		"clarkdgh",
		"just4test",
		"github_app.txt",
		&github.RepositoryContentFileOptions{
			Message: Str2Ptr("message from github app"),
			Content: []byte("this is the content from github app"),
			SHA:     fileContent.SHA,
		},
	)
	if err != nil {
		panic(err)
	}
	fmt.Println(c)
}

func commitOnBehalfOfUser() {
	ctx := context.Background()

	conf := &oauth2.Config{
		ClientID:     "Iv1.dea6982c48f35a77",
		ClientSecret: os.Getenv("CLIENT_SECRET"),
		// Scopes:       []string{"SCOPE1", "SCOPE2"},
		Endpoint: oauth2.Endpoint{
			AuthURL:  "https://github.com/login/oauth/authorize",
			TokenURL: "https://github.com/login/oauth/access_token",
		},
	}

	url := conf.AuthCodeURL("state", oauth2.AccessTypeOnline)
	fmt.Printf("Visit the URL for the auth dialog: %v\n", url)

	var code string
	if _, err := fmt.Scan(&code); err != nil {
		log.Fatal(err)
	}
	tok, err := conf.Exchange(ctx, code)
	if err != nil {
		log.Fatal(err)
	}

	oauthClient := conf.Client(ctx, tok)
	userClient := github.NewClient(oauthClient)

	fileContent, _, _, err := userClient.Repositories.GetContents(ctx, "clarkdgh", "just4test", "on_behalf_user.txt", &github.RepositoryContentGetOptions{})
	if err != nil {
		panic(err)
	}

	d, _, err := userClient.Repositories.CreateFile(
		ctx,
		"clarkdgh",
		"just4test",
		"on_behalf_user.txt",
		&github.RepositoryContentFileOptions{
			Message: Str2Ptr("message from behalf user"),
			Content: []byte("this is the content from behalf user"),
			SHA:     fileContent.SHA,
		},
	)
	if err != nil {
		panic(err)
	}
	fmt.Println(d)
}

func getUserByOauth2() {
	ctx := context.Background()
	conf := &oauth2.Config{
		ClientID:     "Iv1.dea6982c48f35a77",
		ClientSecret: os.Getenv("CLIENT_SECRET"),
		// Scopes:       []string{"SCOPE1", "SCOPE2"},
		Endpoint: oauth2.Endpoint{
			AuthURL:  "https://github.com/login/oauth/authorize",
			TokenURL: "https://github.com/login/oauth/access_token",
		},
	}

	url := conf.AuthCodeURL("state", oauth2.AccessTypeOnline)
	fmt.Printf("Visit the URL for the auth dialog: %v\n", url)

	var code string
	if _, err := fmt.Scan(&code); err != nil {
		log.Fatal(err)
	}
	tok, err := conf.Exchange(ctx, code)
	if err != nil {
		log.Fatal(err)
	}

	oauthClient := conf.Client(ctx, tok)
	userClient := github.NewClient(oauthClient)

	user, _, err := userClient.Users.Get(ctx, "")
	if err != nil {
		panic(err)
	}
	fmt.Println(user)
}

func makeDiff() {
	text1 := `Lorem ipsum dolor.
aaaa
fdfas
fdfd
df32432
dfdsa
cfd
`
	text2 := `Lorem dolor sit amet.
aaaa
fdfas
fdfd
df32432
dfdsa
bbb`
	dmp := diffmatchpatch.New()

	// chars1, chars2, lines := dmp.DiffLinesToChars(text1, text2)
	// fmt.Println(chars1, chars2, lines)

	// diffs := dmp.DiffMain(text1, text2, true)
	// fmt.Println(dmp.DiffPrettyText(diffs))
	// fmt.Println(dmp.DiffText1(diffs))
	// fmt.Println(dmp.DiffText2(diffs))
	// fmt.Println(dmp.DiffPrettyHtml(diffs))
	// fmt.Println(dmp.DiffToDelta(diffs))

	// diffs := dmp.DiffHalfMatch(text1, text2)
	// fmt.Println(diffs)

	patchs := dmp.PatchMake(text1, text2)
	for _, patch := range patchs {
		fmt.Println(patch.String())
	}
}

func getUserByInstallation() {
	// Shared transport to reuse TCP connections.
	tr := http.DefaultTransport
	// Wrap the shared transport for use with the app ID 1 authenticating with installation ID 99.
	// itr, err := ghinstallation.NewKeyFromFile(tr, 315666, 36256853, "/Users/wenjiazhi/playground/wenjiazhi.2023-04-09.private-key.pem")
	itr, err := ghinstallation.NewAppsTransportKeyFromFile(tr, 315666, "/Users/wenjiazhi/playground/wenjiazhi.2023-04-09.private-key.pem")
	if err != nil {
		log.Fatal(err)
	}
	// Use installation transport with github.com/google/go-github
	client := github.NewClient(&http.Client{Transport: itr})
	ctx := context.Background()
	installation, _, err := client.Apps.GetInstallation(ctx, 36392445)
	if err != nil {
		panic(err)
	}
	fmt.Println(installation)
	// ins, err := json.Marshal(installation)
	// if err != nil {
	// 	panic(err)
	// }
	// fmt.Println(string(ins))
}

func ListReposByInstallation() {
	// Shared transport to reuse TCP connections.
	tr := http.DefaultTransport
	// Wrap the shared transport for use with the app ID 1 authenticating with installation ID 99.
	itr, err := ghinstallation.NewKeyFromFile(tr, 315666, 36392445, "/Users/wenjiazhi/playground/wenjiazhi.2023-04-09.private-key.pem")
	// itr, err := ghinstallation.NewAppsTransportKeyFromFile(tr, 315666, "/Users/wenjiazhi/playground/wenjiazhi.2023-04-09.private-key.pem")
	if err != nil {
		log.Fatal(err)
	}
	// Use installation transport with github.com/google/go-github
	client := github.NewClient(&http.Client{Transport: itr})
	ctx := context.Background()
	repos, _, err := client.Apps.ListRepos(ctx, &github.ListOptions{})
	if err != nil {
		panic(err)
	}
	fmt.Println(repos)
}

func main() {
	// commitAsGithubApp()
	// commitOnBehalfOfUser()
	// getUserByOauth2()
	makeDiff()
	// getUserByInstallation()
	// ListReposByInstallation()
}
