package main

import (
	"context"
	"flag"
	"fmt"

	metav1 "k8s.io/apimachinery/pkg/apis/meta/v1"
	"k8s.io/client-go/kubernetes"
	"k8s.io/client-go/tools/clientcmd"
)

func main() {
	kubeConfig := flag.String("kubeconfig", "/Users/yy/.kube/config", "kube config file")
	flag.Parse()

	config, err := clientcmd.BuildConfigFromFlags("", *kubeConfig)
	if err != nil {
		panic(err)
	}
	clientset, err := kubernetes.NewForConfig(config)
	if err != nil {
		panic(err)
	}

	pods, err := clientset.CoreV1().Pods("default").Get(context.Background(), "nginx-deployment-9456bbbf9-cw68b", metav1.GetOptions{})
	if err != nil {
		panic(err)
	}
	fmt.Printf("%v\n", pods.Status)
}
