/*
Copyright 2023.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

package controller

import (
	"context"

	metav1 "k8s.io/apimachinery/pkg/apis/meta/v1"
	"k8s.io/apimachinery/pkg/runtime"
	"k8s.io/apimachinery/pkg/types"
	ctrl "sigs.k8s.io/controller-runtime"
	"sigs.k8s.io/controller-runtime/pkg/client"
	"sigs.k8s.io/controller-runtime/pkg/log"

	cmngr "github.com/cert-manager/cert-manager/pkg/apis/certmanager/v1"
	networking "k8s.io/api/networking/v1"
	batchv1 "tutorial.kubebuilder.io/project/api/v1"
)

// CustomDomainReconciler reconciles a CustomDomain object
type CustomDomainReconciler struct {
	client.Client
	Scheme *runtime.Scheme
}

//+kubebuilder:rbac:groups=batch.tutorial.kubebuilder.io,resources=customdomains,verbs=get;list;watch;create;update;patch;delete
//+kubebuilder:rbac:groups=batch.tutorial.kubebuilder.io,resources=customdomains/status,verbs=get;update;patch
//+kubebuilder:rbac:groups=batch.tutorial.kubebuilder.io,resources=customdomains/finalizers,verbs=update

// Reconcile is part of the main kubernetes reconciliation loop which aims to
// move the current state of the cluster closer to the desired state.
// TODO(user): Modify the Reconcile function to compare the state specified by
// the CustomDomain object against the actual cluster state, and then
// perform operations to make the cluster state reflect the state specified by
// the user.
//
// For more details, check Reconcile and its Result here:
// - https://pkg.go.dev/sigs.k8s.io/controller-runtime@v0.16.0/pkg/reconcile
func (r *CustomDomainReconciler) Reconcile(ctx context.Context, req ctrl.Request) (ctrl.Result, error) {
	log := log.FromContext(ctx)
	log.V(1).Info("customdomain reconcile start")

	var customDomain batchv1.CustomDomain
	if err := r.Get(ctx, req.NamespacedName, &customDomain); err != nil {
		log.Error(err, "unable to fetch customDomain")
		return ctrl.Result{}, client.IgnoreNotFound(err)
	}

	// ingress, err := r.constructIngressForCustomDomain(req, &customDomain)
	// if err != nil {
	// 	log.Error(err, "unable to construct ingress")
	// 	return ctrl.Result{}, err
	// }
	// if err := r.Create(ctx, ingress); err != nil {
	// 	log.Error(err, "create ingress failed")
	// 	return ctrl.Result{}, err
	// }
	var certificate cmngr.Certificate
	if err := r.Get(ctx, types.NamespacedName{Namespace: req.Namespace, Name: "quickstart-example-tls"}, &certificate); err != nil {
		log.Error(err, "unable to fetch certificate")
		return ctrl.Result{}, client.IgnoreNotFound(err)
	}
	log.V(1).Info("get certificate success", "cert", certificate)

	return ctrl.Result{}, nil
}

func (r *CustomDomainReconciler) constructIngressForCustomDomain(req ctrl.Request, customDomain *batchv1.CustomDomain) (*networking.Ingress, error) {
	pathType := networking.PathTypePrefix
	ingress := networking.Ingress{
		ObjectMeta: metav1.ObjectMeta{
			Name:      "test",
			Namespace: req.Namespace,
		},
		Spec: networking.IngressSpec{
			Rules: []networking.IngressRule{{
				Host: customDomain.Spec.Host,
				IngressRuleValue: networking.IngressRuleValue{
					HTTP: &networking.HTTPIngressRuleValue{
						Paths: []networking.HTTPIngressPath{{
							Path:     "/",
							PathType: &pathType,
							Backend: networking.IngressBackend{
								Service: &networking.IngressServiceBackend{
									Name: "go-httpbin",
									Port: networking.ServiceBackendPort{
										Number: 8080,
									},
								},
							},
						}},
					},
				},
			}},
		},
	}
	if err := ctrl.SetControllerReference(customDomain, &ingress, r.Scheme); err != nil {
		return nil, err
	}
	return &ingress, nil
}

// SetupWithManager sets up the controller with the Manager.
func (r *CustomDomainReconciler) SetupWithManager(mgr ctrl.Manager) error {
	return ctrl.NewControllerManagedBy(mgr).
		For(&batchv1.CustomDomain{}).
		Complete(r)
}
