#!/usr/bin/env python3
"""Launch one isolated, bounded classic-tool worker; uses the caller's AWS CLI identity."""
import argparse,datetime,json,subprocess
from pathlib import Path
p=argparse.ArgumentParser()
p.add_argument('--tool',choices=['bwa','blast','minimap2'],required=True)
p.add_argument('--region',default='eu-west-2');p.add_argument('--ami',required=True)
p.add_argument('--subnet',required=True);p.add_argument('--security-group',required=True)
p.add_argument('--ssh-public-key',type=Path,required=True);p.add_argument('--hours',type=int,default=3)
p.add_argument('--spot',action='store_true');p.add_argument('--instance-type',default='c7i.2xlarge');p.add_argument('--volume-gib',type=int,default=50)
a=p.parse_args();assert 1<=a.hours<=6
stamp=datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S')
root=Path('data/ec2/jobs')/(stamp+'-'+a.tool);root.mkdir(parents=True)
packages={'bwa':'bwa','blast':'ncbi-blast+','minimap2':'minimap2'}
cloud={'ssh_authorized_keys':[a.ssh_public_key.read_text().strip()], 'package_update':True,'packages':['python3','curl',packages[a.tool]],'runcmd':[['shutdown','-P','+'+str(a.hours*60),'Bounded isolated ooff benchmark worker'],['install','-d','-o','ubuntu','-g','ubuntu','/home/ubuntu/ooff']]}
user_data=root/'cloud-init.yaml';user_data.write_text('#cloud-config\n'+json.dumps(cloud,indent=2)+'\n')
payload={'ImageId':a.ami,'InstanceType':a.instance_type,'MinCount':1,'MaxCount':1,'SubnetId':a.subnet,'SecurityGroupIds':[a.security_group],'InstanceInitiatedShutdownBehavior':'stop','MetadataOptions':{'HttpTokens':'required'},'BlockDeviceMappings':[{'DeviceName':'/dev/sda1','Ebs':{'VolumeSize':a.volume_gib,'VolumeType':'gp3','Encrypted':True,'DeleteOnTermination':False}}],'TagSpecifications':[{'ResourceType':kind,'Tags':[{'Key':'Project','Value':'ooff'},{'Key':'Name','Value':f'ooff-classic-{a.tool}-{stamp}'}]} for kind in ['instance','volume']]}
if a.spot:payload['InstanceMarketOptions']={'MarketType':'spot','SpotOptions':{'SpotInstanceType':'persistent','InstanceInterruptionBehavior':'stop'}}
request=root/'launch.json';request.write_text(json.dumps(payload,indent=2)+'\n')
cmd=['aws','ec2','run-instances','--region',a.region,'--cli-input-json','file://'+str(request),'--user-data','file://'+str(user_data),'--output','json']
response=subprocess.run(cmd,text=True,capture_output=True)
(root/'launch.stdout').write_text(response.stdout);(root/'launch.stderr').write_text(response.stderr)
(root/'launch-result.json').write_text(json.dumps(dict(command=cmd,returncode=response.returncode),indent=2)+'\n')
if response.returncode:
    print(response.stderr);raise SystemExit(response.returncode)
result=json.loads(response.stdout);(root/'instance.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(dict(tool=a.tool,instance_id=result['Instances'][0]['InstanceId'],artifacts=str(root))))
