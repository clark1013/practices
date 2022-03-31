package main

import (
	"context"
	"flag"
	"fmt"
	"time"

	metav1 "k8s.io/apimachinery/pkg/apis/meta/v1"
	"k8s.io/apimachinery/pkg/util/wait"
	"k8s.io/client-go/informers"
	"k8s.io/client-go/kubernetes"
	"k8s.io/client-go/tools/cache"
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

	pods, err := clientset.CoreV1().Pods("default").List(context.Background(), metav1.ListOptions{LabelSelector: "app=nginx"})
	if err != nil {
		panic(err)
	}
	fmt.Printf("%v\n", len(pods.Items))

	informerFactory := informers.NewSharedInformerFactory(clientset, time.Second*5)
	podInformer := informerFactory.Core().V1().Pods()
	podInformer.Informer().AddEventHandler(cache.ResourceEventHandlerFuncs{
		AddFunc:    func(obj interface{}) { println("add") },
		UpdateFunc: func(oldObj, newObj interface{}) { println("update") },
		DeleteFunc: func(obj interface{}) { println("delete") },
	})
	informerFactory.Start(wait.NeverStop)
	informerFactory.WaitForCacheSync(wait.NeverStop)
	for {

	}
}
