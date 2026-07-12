module "my_eks" {
  source = "./modules/eks"
  cluster_name = "test-cluster"
}
